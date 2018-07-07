use std::clone::Clone;

pub trait ChannelState: Clone {
  type Delta;
  fn initial_state() -> Self;
  fn transform(self: Self, delta: Self::Delta) -> Self;
  fn signal_at(self: &Self, tick: u64) -> f32;
}

pub enum PulseDelta {
    SetFrameCount(u64),
    SetVolume(u16),
    SetPulse(u16),
    SetEnvelope(u16),
    PlayNote(u16, u16, u16),
}

pub enum TriangleDelta {
    SetFrameCount(u64),
    SetVolume(u16),
    SetEnvelope(u16),
    PlayNote(u16, u16, u16),
}

#[derive(Copy, Clone)]
pub struct PulseState {
    frame_count: u64,
}

#[derive(Copy, Clone)]
pub struct TriangleState {
    frame_count: u64,
}

impl ChannelState for PulseState {
    type Delta = PulseDelta;

    fn initial_state() -> Self {
        PulseState { frame_count: 0 }
    }

    fn transform(self: Self, delta: PulseDelta) -> Self {
        // TODO
        self
    }

    fn signal_at(self: &Self, tick: u64) -> f32 {
        return 0f32;
    }
}

impl ChannelState for TriangleState {
    type Delta = TriangleDelta;

    fn initial_state() -> Self {
        TriangleState { frame_count: 0 }
    }

    fn transform(self: Self, delta: TriangleDelta) -> Self {
        // TODO
        self
    }

    fn signal_at(self: &Self, tick: u64) -> f32 {
        return 0f32;
    }
}
