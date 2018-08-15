use apu::Apu;
use apu::channel::{ApuChannelDelta, WhichPulse};
use apu::channel_differ::{
  ChannelSnapshot, NoiseDiffer, PulseDiffer, TriangleDiffer, APU_CHANNEL_SIZE,
};
use clock::Processor;
use memory::{ReadAddr, WriteAddr};
use std::sync::mpsc::Sender;

const APU_REGISTER_START: usize = 0x4000;
const APU_REGISTER_RANGE: usize = 22;

const REG_PULSE1_ROOT: usize = 0;
const REG_PULSE2_ROOT: usize = REG_PULSE1_ROOT + 4;
const REG_TRIANGLE_ROOT: usize = REG_PULSE2_ROOT + 4;
const REG_NOISE_ROOT: usize = REG_TRIANGLE_ROOT + 4;

type SnapshotRepr = [u8; APU_REGISTER_RANGE];

pub struct ApuImpl {
  previous_snapshot: RegisterSnapshot,
  delta_stream: Sender<ApuChannelDelta>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct RegisterSnapshot {
  /// Includes registers $4000-$4015
  registers: SnapshotRepr,
}

impl ApuImpl {
  pub fn create(sender: Sender<ApuChannelDelta>) -> Self {
    ApuImpl {
      delta_stream: sender,
      previous_snapshot: RegisterSnapshot::default(),
    }
  }
}

impl<T: ReadAddr + WriteAddr> Processor<T> for ApuImpl {
  fn cycle(self: &mut Self, memory: &mut T) {
    let new_snapshot = RegisterSnapshot::create_from_memory(memory);
    let deltas = self.previous_snapshot.diff(&new_snapshot, memory);
    let result = self.delta_stream.send(ApuChannelDelta::Many(deltas));

    if let Result::Err(e) = result {
      panic!("The apu decided to burn the house down, CYA\n\n{:?}", e);
    }

    self.previous_snapshot = new_snapshot;
  }
}

impl ReadAddr for ApuImpl {
    fn read_addr(self: &mut Self, addr: u16) -> u8 {
        0
    }
}

impl WriteAddr for ApuImpl {
    fn write_addr(self: &mut Self, addr: u16, value: u8) -> u8 {
        0
    }
}

impl Apu for ApuImpl {}

impl Default for RegisterSnapshot {
  fn default() -> Self {
    let registers: [u8; APU_REGISTER_RANGE] = [0; APU_REGISTER_RANGE];
    RegisterSnapshot { registers }
  }
}

impl RegisterSnapshot {
  fn create_from_memory<M: ReadAddr>(memory: &mut M) -> Self {
    let mut registers: SnapshotRepr = [0; APU_REGISTER_RANGE];
    for offset in 0..APU_REGISTER_RANGE {
      let read_index = (APU_REGISTER_START + offset) as u16;
      let read_value = memory.read_addr(read_index);
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

  fn diff<M: ReadAddr>(self: &Self, other: &Self, _memory: &M) -> Vec<ApuChannelDelta> {
    use apu::channel::WhichPulse::*;

    let mut changes = vec![];

    self.make_pulse_differ(other, P1).diff(&mut changes);
    self.make_pulse_differ(other, P2).diff(&mut changes);
    self.make_triangle_differ(other).diff(&mut changes);
    self.make_noise_differ(other).diff(&mut changes);
    return changes;
  }

  fn get_channel(self: &Self, root: usize) -> ChannelSnapshot {
    let mut channel_registers = [0; APU_CHANNEL_SIZE];
    channel_registers.clone_from_slice(&self.registers[root..root + 4]);
    return channel_registers;
  }

  fn make_pulse_differ(self: &Self, other: &Self, which: WhichPulse) -> PulseDiffer {
    let channel_offset = match which {
      WhichPulse::P1 => REG_PULSE1_ROOT,
      WhichPulse::P2 => REG_PULSE2_ROOT,
    };

    return PulseDiffer::create(
      self.get_channel(channel_offset),
      other.get_channel(channel_offset),
      match which {
        WhichPulse::P1 => ApuChannelDelta::Pulse1,
        WhichPulse::P2 => ApuChannelDelta::Pulse2,
      },
    );
  }

  fn make_triangle_differ(self: &Self, other: &Self) -> TriangleDiffer {
    return TriangleDiffer::create(
      self.get_channel(REG_TRIANGLE_ROOT),
      other.get_channel(REG_TRIANGLE_ROOT),
    );
  }

  fn make_noise_differ(self: &Self, other: &Self) -> NoiseDiffer {
    return NoiseDiffer::create(
      self.get_channel(REG_NOISE_ROOT),
      other.get_channel(REG_NOISE_ROOT),
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use apu::channel::ApuChannelDelta as A;
  use apu::channel::*;
  use memory::ReadAddr;

  impl ReadAddr for Vec<u8> {
    fn read_addr(&mut self, addr: u16) -> u8 {
      self[addr as usize]
    }
  }

  fn init_memory(cap: usize) -> impl ReadAddr {
    Vec::with_capacity(cap)
  }

  fn init_states(cap: usize) -> (impl ReadAddr, RegisterSnapshot) {
    (init_memory(cap), RegisterSnapshot::default())
  }

  impl RegisterSnapshot {
    fn with(at: usize, value: u8) -> Self {
      let mut r = RegisterSnapshot::default();
      r.registers[at] = value;
      return r;
    }
  }

  #[test]
  fn duty_changes() {
    let channel_conf: [(usize, fn(PulseDelta) -> A); 2] = [(0, A::Pulse1), (4, A::Pulse2)];

    for (offset, make) in channel_conf.iter() {
      let memory = init_memory(0);
      let duty_0 = RegisterSnapshot::with(0 + offset, 0b0000_0000);
      let duty_1 = RegisterSnapshot::with(0 + offset, 0b0100_0000);
      let duty_2 = RegisterSnapshot::with(0 + offset, 0b1000_0000);
      let duty_3 = RegisterSnapshot::with(0 + offset, 0b1100_0000);

      assert_eq!(
        duty_1.diff(&duty_0, &memory),
        vec![make(PulseDelta::SetPulseWidth(PulseWidth::Duty0))],
      );

      assert_eq!(
        duty_0.diff(&duty_1, &memory),
        vec![make(PulseDelta::SetPulseWidth(PulseWidth::Duty1))],
      );

      assert_eq!(
        duty_0.diff(&duty_2, &memory),
        vec![make(PulseDelta::SetPulseWidth(PulseWidth::Duty2))],
      );

      assert_eq!(
        duty_0.diff(&duty_3, &memory),
        vec![make(PulseDelta::SetPulseWidth(PulseWidth::Duty3))],
      );
    }
  }

  #[test]
  fn period_changes() {
    let (memory, initial) = init_states(0);
    let change_1 = RegisterSnapshot::with(REG_PULSE1_ROOT + 2, 0b0000_0001);
    let change_2 = RegisterSnapshot::with(REG_PULSE2_ROOT + 2, 0b0000_0001);
    let change_3 = RegisterSnapshot::with(REG_TRIANGLE_ROOT + 2, 0b0000_0001);

    let mut changes = vec![];
    changes.extend(initial.diff(&change_1, &memory));
    changes.extend(initial.diff(&change_2, &memory));
    changes.extend(initial.diff(&change_3, &memory));

    let expected = vec![
      A::Pulse1(PulseDelta::SetPeriod(1)),
      A::Pulse2(PulseDelta::SetPeriod(1)),
      A::Triangle(TriangleDelta::SetPeriod(1)),
    ];

    assert_eq!(changes, expected);
  }

  #[test]
  fn volume_changes() {
    let (memory, initial) = init_states(0);
    let change_1 = RegisterSnapshot::with(REG_PULSE1_ROOT, 0b0000_0001);
    let change_2 = RegisterSnapshot::with(REG_PULSE2_ROOT, 0b0000_0001);
    let change_3 = RegisterSnapshot::with(REG_NOISE_ROOT, 0b0000_0001);

    let mut changes = vec![];
    changes.extend(initial.diff(&change_1, &memory));
    changes.extend(initial.diff(&change_2, &memory));
    changes.extend(initial.diff(&change_3, &memory));

    let expected = vec![
      A::Pulse1(PulseDelta::SetVolume(1)),
      A::Pulse2(PulseDelta::SetVolume(1)),
      A::Noise(NoiseDelta::SetVolume(1)),
    ];

    assert_eq!(changes, expected);
  }
}
