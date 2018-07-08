//! # Register Information
//!
//! This table is a mapping of the register locations with the roles of said registers.
//!
//!  Registers  | Channels | Units
//! ------------|----------|-----------------------------------------------------------------
//! $4000-$4003 | Pulse 1  | Timer, length counter, envelope, sweep
//! $4004-$4007 | Pulse 2  | Timer, length counter, envelope, sweep
//! $4008-$400B | Triangle | Timer, length counter, linear counter
//! $400C-$400F | Noise    | Timer, length counter, envelope, linear feedback shift register
//! $4010-$4013 | DMC      | Timer, memory reader, sample buffer, output unit
//! $4015       | All      | Channel enable and length counter status
//! $4017       | All      | Frame counter
//!
//! ## Pulse 1 & 2, Binary Protocol
//!
//! ### First Byte, $4000 & $4004
//!
//!  Pulse 1 | Pulse 2 | Legend
//! ---------|---------|-----------
//!   $4000  |  $4004  | DDLC VVVV
//!
//! Below is the pulse shape based on the D bits.
//!
//!  Value | Repr | Percentage | Wave Shape
//! -------|------|------------|-----------------
//!    0   |  00  |    12.5%   | 0 1 0 0 0 0 0 0
//!    1   |  01  |    25%     | 0 1 1 0 0 0 0 0
//!    2   |  10  |    50%     | 0 1 1 1 1 0 0 0
//!    3   |  11  |    75%     | 0 1 1 1 1 1 1 0
//!
//! - *Duty cycle bits (D)*, are used to control the width of the
//!   pulse in the pulse wave. When changed the sequencer's
//!   current position is unaffected. Above is a table of the
//!   pulse shape depending on the bit value. [Read more here][Pulse]
//!
//! - *Length Counter halt (l)* is a boolean value for letting the
//!   NES automatically control the duration of APU wave forms.
//!   When active the envelope loops, when deactived the envelope
//!   level remains the same at the end of the envelope. [Read more here][Env].
//!
//! - *Constant Volume (c)* bit is a binary flag which makes the
//!   waves volume a constant level. [Read more here][Env].
//!
//! - *Volume (V)*, which also servers as the initial value of the
//!   envelope at the start of a wave. [Read more here][Env].
//!
//! ### Second Byte, $4001 & $4005
//!
//!  Pulse 1 | Pulse 2 | Legend
//! ---------|---------|-----------
//!   $4001  |  $4005  | EPPP NSSS
//!
//! ### Third Byte, $4002 & $4006
//!
//!  Pulse 1 | Pulse 2 | Legend
//! ---------|---------|-----------
//!   $4002  |  $4006  | TTTT TTTT
//!
//! ### Forth Byte, $4003 & $4007
//!
//!  Pulse 1 | Pulse 2 | Legend
//! ---------|---------|-----------
//!   $4003  |  $4007  | LLLL LTTT
//!
//! [Pulse]: https://wiki.nesdev.com/w/index.php/APU_Pulse
//! [Env]: https://wiki.nesdev.com/w/index.php/APU_Envelope
//! [Sweep]: https://wiki.nesdev.com/w/index.php/APU_Sweep

use std::clone::Clone;
use rand::{Rng, thread_rng};

#[derive(Copy, Clone, Debug)]
pub enum Envelope {
    Constant(u64),
}

pub struct ChannelTuning {
    pub tick: u64,
    pub base_frequency: f32,
}

pub trait ChannelState: Clone + Default {
  type Delta;
  fn transform(self: Self, delta: Self::Delta) -> Self;
  fn signal_at(self: &Self, config: &ChannelTuning) -> f32;
}

////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ApuChannelDelta {
    Pulse1(PulseDelta),
    Pulse2(PulseDelta),
    Noise(NoiseDelta),
    Triangle(TriangleDelta),
    Many(Vec<ApuChannelDelta>),
}

#[derive(Copy, Clone, Debug)]
pub struct ApuChannelState {
    pub pulse_1: PulseState,
    pub pulse_2: PulseState,
    pub triangle: TriangleState,
    pub noise: NoiseState,
}

impl Default for ApuChannelState {
    fn default() -> Self {
        ApuChannelState {
            pulse_1: PulseState::default(),
            pulse_2: PulseState::default(),
            triangle: TriangleState::default(),
            noise: NoiseState::default(),
        }
    }
}

impl ChannelState for ApuChannelState {
    type Delta = ApuChannelDelta;

    fn transform(self: Self, delta: ApuChannelDelta) -> Self {
        match delta {
            ApuChannelDelta::Pulse1(d) =>
                Self { pulse_1: self.pulse_1.transform(d), ..self },
            ApuChannelDelta::Pulse2(d) =>
                Self { pulse_2: self.pulse_2.transform(d), ..self },
            ApuChannelDelta::Triangle(d) =>
                Self { triangle: self.triangle.transform(d), ..self },
            ApuChannelDelta::Noise(d) =>
                Self { noise: self.noise.transform(d), ..self },
            ApuChannelDelta::Many(deltas) =>
                deltas.into_iter().fold(
                    self,
                    |state, sub_delta| state.transform(sub_delta),
                )
        }
    }

    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        0.0
        + self.pulse_1.signal_at(&config)
        + self.pulse_2.signal_at(&config)
        + self.triangle.signal_at(&config)
        + self.noise.signal_at(&config)
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct PulseState {
    frame_count: u64,
    pulse_width: u8,
    envelope: Envelope,
    frequency: f32,
    volume: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum PulseDelta {
    SetFrameCount(u64),
    SetPulseWidth(u8),
    SetVolume(u8),
    SetEnvelope(Envelope),
}

impl Default for PulseState {
    fn default() -> Self {
        PulseState {
            frame_count: 0,
            volume: 0,
            frequency: 440.0,
            pulse_width: 0,
            envelope: Envelope::Constant(0),
        }
    }
}

impl ChannelState for PulseState {
    type Delta = PulseDelta;

    fn transform(self: Self, delta: PulseDelta) -> Self {
        match delta {
            PulseDelta::SetVolume(v) => Self { volume: v, ..self },
            PulseDelta::SetFrameCount(f) => Self { frame_count: f, ..self },
            PulseDelta::SetPulseWidth(w) => Self { pulse_width: w, ..self },
            PulseDelta::SetEnvelope(e) => Self { envelope: e, ..self },
        }
    }

    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        return 0.0;
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct TriangleState {
    frame_count: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum TriangleDelta {
    SetFrameCount(u8),
    SetVolume(u8),
    SetEnvelope(Envelope),
    PlayNote(u8),
}

impl Default for TriangleState {
    fn default() -> Self {
        TriangleState { frame_count: 0 }
    }
}

impl ChannelState for TriangleState {
    type Delta = TriangleDelta;

    fn transform(self: Self, delta: TriangleDelta) -> Self {
        // TODO
        self
    }

    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        return 0.0;
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct NoiseState {
    volume: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum NoiseDelta {
    SetVolume(u8),
}

impl Default for NoiseState {
    fn default() -> Self {
        NoiseState { volume: 0 }
    }
}

impl ChannelState for NoiseState {
    type Delta = NoiseDelta;

    fn transform(self: Self, delta: NoiseDelta) -> Self {
        match delta {
            NoiseDelta::SetVolume(v) => Self { volume: v, ..self },
        }
    }

    fn signal_at(self: &Self, _config: &ChannelTuning) -> f32 {
        match self.volume {
            0 => 0.0,
            volume => {
                let amplitude = (volume as f32) / (u8::max_value() as f32);
                let mut random = thread_rng();
                random.gen_range(-amplitude, amplitude)
            }
        }
    }
}
