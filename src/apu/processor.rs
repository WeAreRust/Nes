use apu::channel::{ApuChannelDelta, PulseWidth, PulseDelta};
use memory::Memory;
use clock::Executable;
use std::sync::mpsc::{Sender};

const APU_REGISTER_START: usize = 0x4000;
const APU_REGISTER_RANGE: usize = 22;

const REG_PULSE1_ROOT: usize = 0;
const REG_PULSE2_ROOT: usize = 4;
const REG_TRIANGLE_ROOT: usize = 8;
const REG_NOISE_ROOT: usize = 12;

const REG_PULSE1_VOL_OFFSET: usize = 0;
const REG_PULSE1_SWEEP_OFFSET: usize = 1;
const REG_PULSE1_LO_OFFSET: usize = 2;
const REG_PULSE1_HI_OFFSET: usize = 3;
const REG_PULSE2_VOL_OFFSET: usize = 4;
const REG_PULSE2_SWEEP_OFFSET: usize = 5;
const REG_PULSE2_LO_OFFSET: usize = 6;
const REG_PULSE2_HI_OFFSET: usize = 7;

type SnapshotRepr = [u8; APU_REGISTER_RANGE];

pub struct APU {
    previous_snapshot: RegisterSnapshot,
    delta_stream: Sender<ApuChannelDelta>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct RegisterSnapshot {
    /// Includes registers $4000-$4015
    registers: SnapshotRepr,
}

struct Differ<'a> {
    changes: Vec<ApuChannelDelta>,
    memory: &'a Memory,
    old: &'a RegisterSnapshot,
    new: &'a RegisterSnapshot,
}

impl APU {
    fn create(sender: Sender<ApuChannelDelta>) -> Self {
        APU {
            delta_stream: sender,
            previous_snapshot: RegisterSnapshot::default(),
        }
    }
}

impl Executable for APU {
    fn execute(self: &mut Self, memory: &mut Memory) {
        let new_snapshot = RegisterSnapshot::create_from_memory(memory);
        let deltas = self.previous_snapshot.diff(&new_snapshot, memory);
        let result = self.delta_stream.send(ApuChannelDelta::Many(deltas));

        if let Result::Err(e) = result {
            panic!("The apu decided to burn the house down, CYA\n{:?}", e);
        }
    }
}

impl Default for RegisterSnapshot {
    fn default() -> Self {
        let registers: [u8; APU_REGISTER_RANGE] = [0; APU_REGISTER_RANGE];
        RegisterSnapshot { registers }
    }
}

impl RegisterSnapshot {
    fn create_from_memory(memory: &Memory) -> Self {
        let mut registers: SnapshotRepr = [0; APU_REGISTER_RANGE];
        for offset in 0..APU_REGISTER_RANGE {
            let read_index = (APU_REGISTER_START + offset) as u16;
            let read_value = memory.fetch(read_index);
            registers[offset] = read_value;
        }
        RegisterSnapshot { registers }
    }

    fn create_from_repr(registers: SnapshotRepr) -> Self {
        RegisterSnapshot { registers }
    }

    fn has_changed_at(self: &Self, other: &Self, at: usize) -> bool {
        return self.registers[at] != other.registers[at];
    }

    fn diff(self: &Self, other: &Self, memory: &Memory) -> Vec<ApuChannelDelta> {
        let mut differ = Differ::create(self, other, memory);
        differ.diff();
        return differ.get_changes();
    }
}

impl<'a> Differ<'a> {
    fn create(old: &'a RegisterSnapshot, new: &'a RegisterSnapshot, memory: &'a Memory) -> Self {
        let changes = vec![];
        Differ { changes, old, new, memory }
    }

    pub fn diff(self: &mut Self) {
        self.diff_pulse_width(REG_PULSE1_ROOT, ApuChannelDelta::Pulse1);
        self.diff_pulse_width(REG_PULSE2_ROOT, ApuChannelDelta::Pulse2);
    }

    fn get_changes(self: Self) -> Vec<ApuChannelDelta> {
        return self.changes;
    }

    fn diff_pulse_width<T>(self: &mut Self, channel_offset: usize, make: T)
            where T: Fn(PulseDelta) -> ApuChannelDelta {
        let pulse_byte_offset = 0;
        let register_offset = pulse_byte_offset + channel_offset;

        if self.old.has_changed_at(self.new, register_offset) {
            let a = self.old.registers[register_offset];
            let b = self.new.registers[register_offset];
            if a & 0b1100_0000 != b & 0b1100_0000 {
                let pulse_width = PulseWidth::calculate(b);
                self.changes.push(make(PulseDelta::SetPulseWidth(pulse_width)));
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use memory::Memory;
    use apu::channel::*;
    use apu::channel::ApuChannelDelta as A;

    fn init_memory(cap: usize) -> Memory {
        Memory::with_bytes(BytesMut::with_capacity(cap))
    }

    fn init_states(cap: usize) -> (Memory, RegisterSnapshot) {
        (init_memory(cap), RegisterSnapshot::default())
    }

    impl RegisterSnapshot {
        fn with(at: usize, value: u8) -> Self {
            let mut r = RegisterSnapshot::default();
            r.registers[at] = value;
            return r
        }
    }

    #[test]
    fn duty_changed_to_0() {
        let memory = init_memory(0);
        let initial = RegisterSnapshot::with(REG_PULSE1_VOL_OFFSET, 0b1000_0000);
        let changed = RegisterSnapshot::with(REG_PULSE1_VOL_OFFSET, 0b0000_0000);

        assert_eq!(
            initial.diff(&changed, &memory),
            vec![A::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty0))],
        );
    }

    #[test]
    fn duty_changed_to_1() {
        let (memory, initial) = init_states(0);
        let changed = RegisterSnapshot::with(REG_PULSE1_VOL_OFFSET, 0b0100_0000);

        assert_eq!(
            initial.diff(&changed, &memory),
            vec![A::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty1))],
        );
    }

    #[test]
    fn duty_changed_to_2() {
        let (memory, initial) = init_states(0);
        let changed = RegisterSnapshot::with(REG_PULSE1_VOL_OFFSET, 0b1000_0000);

        assert_eq!(
            initial.diff(&changed, &memory),
            vec![A::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty2))],
        );
    }

    #[test]
    fn duty_changed_to_3() {
        let (memory, initial) = init_states(0);
        let changed = RegisterSnapshot::with(REG_PULSE1_VOL_OFFSET, 0b1100_0000);

        assert_eq!(
            initial.diff(&changed, &memory),
            vec![A::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty3))],
        );
    }
}
