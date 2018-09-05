use apu::channel::{ApuChannelDelta, NoiseDelta, PulseDelta, PulseWidth, TriangleDelta};

type Deltas = Vec<ApuChannelDelta>;

pub const APU_CHANNEL_SIZE: usize = 4;
pub type ChannelSnapshot = [u8; APU_CHANNEL_SIZE];

pub struct PulseDiffer {
  make_pulse_delta: fn(PulseDelta) -> ApuChannelDelta,
  old_registers: ChannelSnapshot,
  new_registers: ChannelSnapshot,
}

pub struct TriangleDiffer {
  old_registers: ChannelSnapshot,
  new_registers: ChannelSnapshot,
}

pub struct NoiseDiffer {
  old_registers: ChannelSnapshot,
  new_registers: ChannelSnapshot,
}

trait ChannelDiffer {
  type DeltaInternal;

  fn make_delta(self: &Self, internal: Self::DeltaInternal) -> ApuChannelDelta;
  fn get_old_registers(self: &Self) -> &ChannelSnapshot;
  fn get_new_registers(self: &Self) -> &ChannelSnapshot;

  /// Checks if a register has changed at a certain byte
  /// under a certain byte mask, between the old and new
  /// registers.
  fn get_changes(self: &Self, byte: usize, mask: u8) -> Option<u8> {
    let old = read(self.get_old_registers(), byte, mask);
    let new = read(self.get_new_registers(), byte, mask);
    return if new != old { Some(new) } else { None };
  }

  fn add_delta(self: &Self, changes: &mut Deltas, maybe_delta: Option<Self::DeltaInternal>) {
    if let Some(delta) = maybe_delta {
      changes.push(self.make_delta(delta));
    }
  }
}

trait CommonVolumeDiffer: ChannelDiffer {
  fn diff_volume(self: &Self) -> Option<u8> {
    const REGISTER: usize = 0;
    const MASK: u8 = 0b0000_1111;

    return self.get_changes(REGISTER, MASK);
  }
}

trait CommonPeriodDiffer: ChannelDiffer {
  fn diff_period(self: &Self) -> Option<u16> {
    const LO_REGISTER: usize = 2;
    const LO_MASK: u8 = 0b1111_1111;

    const HI_REGISTER: usize = 3;
    const HI_SHIFT: u16 = 3;
    const HI_MASK: u8 = 0b1110_0000;

    let lo = self.get_changes(LO_REGISTER, LO_MASK);
    let hi = self.get_changes(HI_REGISTER, HI_MASK);

    if lo.is_none() && hi.is_none() {
      return None;
    }

    let old_registers = self.get_old_registers();
    let lo = lo.unwrap_or(read(old_registers, LO_REGISTER, LO_MASK)) as u16;
    let hi = hi.unwrap_or(read(old_registers, HI_REGISTER, HI_MASK)) as u16;
    return Some((hi << HI_SHIFT) + lo);
  }
}

impl ChannelDiffer for PulseDiffer {
  type DeltaInternal = PulseDelta;

  fn make_delta(self: &Self, delta: Self::DeltaInternal) -> ApuChannelDelta {
    (self.make_pulse_delta)(delta)
  }

  fn get_old_registers(self: &Self) -> &ChannelSnapshot {
    &self.old_registers
  }

  fn get_new_registers(self: &Self) -> &ChannelSnapshot {
    &self.new_registers
  }
}

impl ChannelDiffer for TriangleDiffer {
  type DeltaInternal = TriangleDelta;

  fn make_delta(self: &Self, delta: Self::DeltaInternal) -> ApuChannelDelta {
    ApuChannelDelta::Triangle(delta)
  }

  fn get_old_registers(self: &Self) -> &ChannelSnapshot {
    &self.old_registers
  }

  fn get_new_registers(self: &Self) -> &ChannelSnapshot {
    &self.new_registers
  }
}

impl ChannelDiffer for NoiseDiffer {
  type DeltaInternal = NoiseDelta;

  fn make_delta(self: &Self, delta: Self::DeltaInternal) -> ApuChannelDelta {
    ApuChannelDelta::Noise(delta)
  }

  fn get_old_registers(self: &Self) -> &ChannelSnapshot {
    &self.old_registers
  }

  fn get_new_registers(self: &Self) -> &ChannelSnapshot {
    &self.new_registers
  }
}

impl CommonVolumeDiffer for PulseDiffer {}
impl CommonVolumeDiffer for NoiseDiffer {}
impl CommonPeriodDiffer for PulseDiffer {}
impl CommonPeriodDiffer for TriangleDiffer {}

impl PulseDiffer {
  pub fn create(old: [u8; 4], new: [u8; 4], make_delta: fn(PulseDelta) -> ApuChannelDelta) -> Self {
    PulseDiffer {
      make_pulse_delta: make_delta,
      old_registers: old,
      new_registers: new,
    }
  }

  pub fn diff(self: &Self, changes: &mut Deltas) {
    self.add_delta(changes, self.diff_pulse_width());
    self.add_delta(changes, self.diff_period().map(PulseDelta::SetPeriod));
    self.add_delta(changes, self.diff_volume().map(PulseDelta::SetVolume));
  }

  fn diff_pulse_width(self: &Self) -> Option<PulseDelta> {
    const PULSE_REGISTER: usize = 0;
    const PULSE_MASK: u8 = 0b1100_0000;

    return self
      .get_changes(PULSE_REGISTER, PULSE_MASK)
      .map(|change| PulseDelta::SetPulseWidth(PulseWidth::calculate(change)));
  }
}

impl TriangleDiffer {
  pub fn create(old_registers: [u8; 4], new_registers: [u8; 4]) -> Self {
    TriangleDiffer {
      old_registers,
      new_registers,
    }
  }

  pub fn diff(self: &Self, changes: &mut Deltas) {
    self.add_delta(changes, self.diff_period().map(TriangleDelta::SetPeriod));
  }
}

impl NoiseDiffer {
  pub fn create(old_registers: [u8; 4], new_registers: [u8; 4]) -> Self {
    NoiseDiffer {
      old_registers,
      new_registers,
    }
  }

  pub fn diff(self: &Self, changes: &mut Deltas) {
    self.add_delta(changes, self.diff_volume().map(NoiseDelta::SetVolume));
  }
}

fn read(snapshot: &ChannelSnapshot, byte: usize, mask: u8) -> u8 {
  snapshot[byte] & mask
}

#[cfg(test)]
mod tests {
  use super::*;
  use apu::channel::*;

  fn make_pulse() -> PulseDiffer {
    let old = [0; APU_CHANNEL_SIZE];
    let new = [0; APU_CHANNEL_SIZE];
    return PulseDiffer::create(old, new, ApuChannelDelta::Pulse1);
  }

  fn make_triangle() -> TriangleDiffer {
    let old = [0; APU_CHANNEL_SIZE];
    let new = [0; APU_CHANNEL_SIZE];
    return TriangleDiffer::create(old, new);
  }

  fn make_noise() -> NoiseDiffer {
    let old = [0; APU_CHANNEL_SIZE];
    let new = [0; APU_CHANNEL_SIZE];
    return NoiseDiffer::create(old, new);
  }

  impl PulseDiffer {
    fn set_old(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.old_registers[at] = value;
      return self;
    }
    fn set_new(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.new_registers[at] = value;
      return self;
    }
  }
  impl TriangleDiffer {
    fn set_old(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.old_registers[at] = value;
      return self;
    }
    fn set_new(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.new_registers[at] = value;
      return self;
    }
  }
  impl NoiseDiffer {
    fn set_old(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.old_registers[at] = value;
      return self;
    }
    fn set_new(self: &mut Self, at: usize, value: u8) -> &mut Self {
      self.new_registers[at] = value;
      return self;
    }
  }

  #[test]
  fn duty_changed_to_0() {
    let change = make_pulse()
      .set_old(0, 0b1000_0000)
      .set_new(0, 0b0000_0000)
      .diff_pulse_width();

    assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty0)));
  }

  #[test]
  fn duty_changed_to_1() {
    let change = make_pulse()
      .set_old(0, 0b0000_0000)
      .set_new(0, 0b0100_0000)
      .diff_pulse_width();

    assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty1)));
  }

  #[test]
  fn duty_changed_to_2() {
    let change = make_pulse()
      .set_old(0, 0b0000_0000)
      .set_new(0, 0b1000_0000)
      .diff_pulse_width();

    assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty2)));
  }

  #[test]
  fn duty_changed_to_3() {
    let change = make_pulse()
      .set_old(0, 0b0000_0000)
      .set_new(0, 0b1100_0000)
      .diff_pulse_width();

    assert_eq!(change, Some(PulseDelta::SetPulseWidth(PulseWidth::Duty3)));
  }

  #[test]
  fn period_change_when_hi_unchanged_but_lo_is() {
    let change = make_pulse()
      .set_old(2, 0b0000_0000)
      .set_new(2, 0b0000_0001)
      // stuff that should be ignored by mask
      .set_old(3, 0b0001_0101)
      .set_new(3, 0b0000_0001)
      .diff_period();
    assert_eq!(change, Some(1));
  }

  #[test]
  fn period_change_when_lo_unchanged_but_hi_is() {
    let change = make_pulse()
      .set_old(3, 0b0000_0000)
      .set_new(3, 0b0010_0000)
      .diff_period();
    assert_eq!(change, Some(1 << 8));
  }

  #[test]
  fn period_changes_when_both_change() {
    let change = make_pulse()
      .set_old(2, 0b0000_0000)
      .set_new(2, 0b0000_0001)
      .set_old(3, 0b0000_0000)
      .set_new(3, 0b0010_0000)
      .diff_period();
    assert_eq!(change, Some((1 << 8) + 1));
  }

  #[test]
  fn period_change_when_nothing_changes() {
    let change = make_pulse().diff_period();
    assert_eq!(change, None);
  }

  #[test]
  fn volume_changes_with_update() {
    let change = make_pulse()
      .set_old(0, 0b0000_0000)
      .set_new(0, 0b0000_0001)
      .diff_volume();
    assert_eq!(change, Some(1));
  }

  #[test]
  fn volume_changes_with_no_update() {
    let change = make_pulse()
      .set_old(0, 0b0000_0000)
      .set_new(0, 0b0000_0000)
      .diff_volume();
    assert_eq!(change, None);
  }
}
