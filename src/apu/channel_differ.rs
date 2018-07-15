use apu::channel::{ApuChannelDelta, PulseWidth};
use memory::Memory;

pub const APU_CHANNEL_SIZE: usize = 4;
pub type ChannelSnapshot = [u8; APU_CHANNEL_SIZE];

pub struct ChannelDiffer<D> {
    old_registers: ChannelSnapshot,
    new_registers: ChannelSnapshot,
    make_delta: fn(D) -> ApuChannelDelta,
    pub set_pulse: Option<fn(PulseWidth) -> D>,
    pub set_period: Option<fn(u16) -> D>,
}

type Deltas = Vec<ApuChannelDelta>;

impl<D> ChannelDiffer<D> {
    pub fn create(old: [u8; 4], new: [u8; 4], make_delta: fn(D) -> ApuChannelDelta) -> Self {
        ChannelDiffer {
            old_registers: old,
            new_registers: new,
            make_delta: make_delta,
            set_pulse: None,
            set_period: None,
        }
    }

    pub fn diff<'a>(self: &Self, _memory: &Memory, changes: &'a mut Deltas) -> &'a Deltas {
        self.add_delta(changes, self.diff_pulse_width());
        self.add_delta(changes, self.diff_period());
        return changes;
    }

    /// Checks if a register has changed at a certain byte
    /// under a certain byte mask, between the old and new
    /// registers.
    fn get_changes(self: &Self, byte: usize, mask: u8) -> Option<u8> {
        let old = read(&self.old_registers, byte, mask);
        let new = read(&self.new_registers, byte, mask);
        return if new != old { Some(new) } else { None };
    }

    fn add_delta(self: &Self, changes: &mut Deltas, maybe_delta: Option<D>) {
        if let Some(delta) = maybe_delta {
            changes.push((self.make_delta)(delta));
        }
    }

    fn diff_period(self: &Self) -> Option<D> {
        const LO_REGISTER: usize = 2;
        const LO_MASK: u8 = 0b1111_1111;

        const HI_REGISTER: usize = 3;
        const HI_SHIFT: u16 = 3;
        const HI_MASK: u8 = 0b1110_0000;

        return self.set_period.and_then(|set_period| {
            let lo = self.get_changes(LO_REGISTER, LO_MASK);
            let hi = self.get_changes(HI_REGISTER, HI_MASK);

            if lo.is_none() && hi.is_none() {
                return None;
            }
            let lo = lo.unwrap_or(read(&self.old_registers, LO_REGISTER, LO_MASK)) as u16;
            let hi = (hi.unwrap_or(read(&self.old_registers, HI_REGISTER, HI_MASK)) as u16) << HI_SHIFT;
            return Some(set_period(hi + lo));
        });
    }

    fn diff_pulse_width(self: &Self) -> Option<D> {
        const PULSE_REGISTER: usize = 0;
        const PULSE_MASK: u8 = 0b1100_0000;

        return self.set_pulse.and_then(|set_pulse|
            self.get_changes(PULSE_REGISTER, PULSE_MASK)
                .map(|change| set_pulse(PulseWidth::calculate(change))));
    }
}

fn read(snapshot: &ChannelSnapshot, byte: usize, mask: u8) -> u8 {
    snapshot[byte] & mask
}

#[cfg(test)]
mod tests {
    use super::*;
    use apu::channel::ApuChannelDelta as A;
    use apu::channel::*;
    use bytes::BytesMut;
    use memory::Memory;

    fn init_memory(cap: usize) -> Memory {
        Memory::with_bytes(BytesMut::with_capacity(cap))
    }

    impl<D> ChannelDiffer<D> {
        fn with_constructor(constructor: fn(D) -> A) -> Self {
            let old = [0; APU_CHANNEL_SIZE];
            let new = [0; APU_CHANNEL_SIZE];
            return ChannelDiffer::create(old, new, constructor);
        }

        fn set_old(self: &mut Self, at: usize, value: u8) -> &mut Self {
            self.old_registers[at] = value;
            return self;
        }

        fn set_new(self: &mut Self, at: usize, value: u8) -> &mut Self {
            self.new_registers[at] = value;
            return self;
        }

        fn set_make_period_delta(self: &mut Self, set_period: fn(u16) -> D) -> &mut Self {
            self.set_period = Some(set_period);
            return self;
        }

        fn set_make_pulse_delta(self: &mut Self, set_pulse: fn(PulseWidth) -> D) -> &mut Self {
            self.set_pulse = Some(set_pulse);
            return self;
        }
    }

    #[test]
    fn duty_changed_to_0() {
        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_pulse_delta(PulseDelta::SetPulseWidth)
            .set_old(0, 0b1000_0000)
            .set_new(0, 0b0000_0000)
            .diff_pulse_width();

        assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty0)));
    }

    #[test]
    fn duty_changed_to_1() {
        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_pulse_delta(PulseDelta::SetPulseWidth)
            .set_old(0, 0b0000_0000)
            .set_new(0, 0b0100_0000)
            .diff_pulse_width();

        assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty1)));
    }

    #[test]
    fn duty_changed_to_2() {
        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_pulse_delta(PulseDelta::SetPulseWidth)
            .set_old(0, 0b0000_0000)
            .set_new(0, 0b1000_0000)
            .diff_pulse_width();

        assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty2)));
    }

    #[test]
    fn duty_changed_to_3() {
        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_pulse_delta(PulseDelta::SetPulseWidth)
            .set_old(0, 0b0000_0000)
            .set_new(0, 0b1100_0000)
            .diff_pulse_width();

        assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty3)));
    }

    #[test]
    fn period_change() {
        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_period_delta(PulseDelta::SetPeriod)
            .set_old(2, 0b0000_0000)
            .set_new(2, 0b0000_0001)
            .diff_period();
        assert_eq!(change, Some(PulseDelta::SetPeriod(1)));

        let change = ChannelDiffer::with_constructor(A::Pulse1)
            .set_make_period_delta(PulseDelta::SetPeriod)
            .set_old(3, 0b0000_0000)
            .set_new(3, 0b0010_0000)
            .diff_period();
        assert_eq!(change, Some(PulseDelta::SetPeriod(1 << 8)));
    }
}
