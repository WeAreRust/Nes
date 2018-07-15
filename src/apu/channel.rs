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

use clock::{CPU_PERIOD, MASTER_FREQUENCY};
use rand::{thread_rng, Rng};
use std::clone::Clone;

const MAX_PEROID: u16 = (1 << 12) - 1;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Envelope {
    Constant(u64),
}

pub struct ChannelTuning {
    pub sample: u64,
    pub sample_rate: u32,
}

pub trait ChannelState: Clone + Default {
    type Delta;
    fn transform(self: Self, delta: Self::Delta) -> Self;
    fn signal_at(self: &Self, config: &ChannelTuning) -> f32;
}

pub trait ChannelFrequency {
    fn get_period(self: &Self) -> u16;
    fn get_period_min(self: &Self) -> u16;

    fn get_frequency(self: &Self) -> Option<f32> {
        let period = self.get_period();
        let min = self.get_period_min();
        if period < min || period > MAX_PEROID {
            return None;
        }

        let f_divider = 16.0 / (period as f32 + 1.0);
        return Some(((MASTER_FREQUENCY / CPU_PERIOD) as f32) / f_divider);
    }
}

pub trait ChannelAmplitude {
    fn get_volume(self: &Self) -> u8;

    fn get_amplitude(self: &Self) -> Option<f32> {
        let volume = self.get_volume();
        if volume == 0 {
            return None;
        }
        return Some((volume as f32) / (u8::max_value() as f32));
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ApuChannelDelta {
    Pulse1(PulseDelta),
    Pulse2(PulseDelta),
    Noise(NoiseDelta),
    Triangle(TriangleDelta),
    Many(Vec<ApuChannelDelta>),
}

#[derive(Clone, Debug)]
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
            ApuChannelDelta::Pulse1(d) => Self {
                pulse_1: self.pulse_1.transform(d),
                ..self
            },
            ApuChannelDelta::Pulse2(d) => Self {
                pulse_2: self.pulse_2.transform(d),
                ..self
            },
            ApuChannelDelta::Triangle(d) => Self {
                triangle: self.triangle.transform(d),
                ..self
            },
            ApuChannelDelta::Noise(d) => Self {
                noise: self.noise.transform(d),
                ..self
            },
            ApuChannelDelta::Many(deltas) => deltas
                .into_iter()
                .fold(self, |state, sub_delta| state.transform(sub_delta)),
        }
    }

    /// This is the total signal of all channels from the APU, and emulates
    /// the NES mixers by using lookup algorithm defined [here][mixer].
    ///
    /// [mixer]: https://wiki.nesdev.com/w/index.php/APU_Mixer
    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        // Mixer look up tables as described in the wiki.
        let pulse_table = |n| 95.52 / (8128.0 / n + 100.0);
        let tnd_table = |n| 163.67 / (24329.0 / n + 100.0);

        // Signals as produced by the seperate channels.
        let pulse_1 = self.pulse_1.signal_at(&config);
        let pulse_2 = self.pulse_2.signal_at(&config);
        let triangle = self.triangle.signal_at(&config);
        let dmc = 0.0;
        let noise = self.noise.signal_at(&config);

        let pulse_mix = pulse_table(pulse_1 + pulse_2);
        let tnd_mix = tnd_table(3.0 * triangle + 2.0 * noise + dmc);

        return pulse_mix + tnd_mix;
    }
}

////////////////////////////////////////////////////////////////////////////

const FREQ_CHUNK: f32 = 0.125;

/// Read more about the wave pulse [here].
///
/// [here]: https://wiki.nesdev.com/w/index.php/APU_Pulse
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PulseWidth {
    /// Has a waveform like `0 1 0 0 0 0 0 0` where 12.5%
    /// of the waveform positive.
    Duty0,
    /// Has a waveform like `0 1 1 0 0 0 0 0` where 25%
    /// of the waveform positive.
    Duty1,
    /// Has a waveform like `0 1 1 1 1 0 0 0` where 50%
    /// of the waveform positive.
    Duty2,
    /// Has a waveform like `0 1 1 1 1 0 0 0` where 75%
    /// of the waveform positive.
    Duty3,
}

impl PulseWidth {
    pub fn calculate(byte: u8) -> PulseWidth {
        let masked = (byte & 0b11000000) >> 6;
        if masked == 0 { PulseWidth::Duty0 }
        else if masked == 1 { PulseWidth::Duty1 }
        else if masked == 2 { PulseWidth::Duty2 }
        else { PulseWidth::Duty3 }
    }

    fn pulse_sign(self: &Self, frequency_progress: f32) -> f32 {
        if frequency_progress > 1.0 {
            panic!("expected frequency >= 1");
        }
        return match self {
            // wave: 0 1 0 0 0 0 0 0
            PulseWidth::Duty0 => match frequency_progress {
                f if f < FREQ_CHUNK => -1.0,
                f if f > FREQ_CHUNK * 2.0 => -1.0,
                _ => 1.0,
            },
            // wave: 0 1 1 0 0 0 0 0
            PulseWidth::Duty1 => match frequency_progress {
                f if f < FREQ_CHUNK => -1.0,
                f if f > FREQ_CHUNK * 3.0 => -1.0,
                _ => 1.0,
            },
            // wave: 0 1 1 1 1 0 0 0
            PulseWidth::Duty2 => match frequency_progress {
                f if f < FREQ_CHUNK => -1.0,
                f if f > FREQ_CHUNK * 5.0 => -1.0,
                _ => 1.0,
            },
            // wave: 0 1 1 1 1 1 1 0
            PulseWidth::Duty3 => match frequency_progress {
                f if f < FREQ_CHUNK => -1.0,
                f if f > FREQ_CHUNK * 7.0 => -1.0,
                _ => 1.0,
            },
        };
    }
}

#[derive(Clone, Debug)]
pub struct PulseState {
    frame_count: u64,
    pulse_width: PulseWidth,
    envelope: Envelope,
    period: u16,
    volume: u8,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PulseDelta {
    SetFrameCount(u64),
    SetPulseWidth(PulseWidth),
    SetVolume(u8),
    SetEnvelope(Envelope),
    SetPeriod(u16),
}

impl Default for PulseState {
    fn default() -> Self {
        PulseState {
            frame_count: 0,
            volume: 0,
            period: 0,
            pulse_width: PulseWidth::Duty0,
            envelope: Envelope::Constant(0),
        }
    }
}

/// When the peroid is below `8` the pulse wave is silient.
/// [Read more here][Pitch].
///
/// [Pitch]: https://wiki.nesdev.com/w/index.php/APU#Pulse_.28.244000-4007.29
impl ChannelFrequency for PulseState {
    fn get_period(self: &Self) -> u16 {
        self.period
    }
    fn get_period_min(self: &Self) -> u16 {
        8
    }
}

impl ChannelAmplitude for PulseState {
    fn get_volume(self: &Self) -> u8 {
        self.volume
    }
}

impl ChannelState for PulseState {
    type Delta = PulseDelta;

    fn transform(self: Self, delta: PulseDelta) -> Self {
        match delta {
            PulseDelta::SetPeriod(p) => Self { period: p, ..self },
            PulseDelta::SetVolume(v) => Self { volume: v, ..self },
            PulseDelta::SetFrameCount(f) => Self {
                frame_count: f,
                ..self
            },
            PulseDelta::SetPulseWidth(w) => Self {
                pulse_width: w,
                ..self
            },
            PulseDelta::SetEnvelope(e) => Self {
                envelope: e,
                ..self
            },
        }
    }

    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        let amplitude = match self.get_amplitude() {
            None => return 0.0,
            Some(a) => a,
        };

        let frequency = match self.get_frequency() {
            None => return 0.0,
            Some(f) => f,
        };

        let sample_offset = config.sample * (config.sample_rate as u64);
        let sample_mod = (sample_offset % frequency as u64) as f32;
        let frequent_percent = sample_mod / frequency;
        let signal = amplitude * self.pulse_width.pulse_sign(frequent_percent);
        return signal;
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct TriangleState {
    period: u16,
    control_flag: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TriangleDelta {
    SetPeriod(u16),
    SetControlFlag(bool),
}

impl Default for TriangleState {
    fn default() -> Self {
        TriangleState {
            period: 0,
            control_flag: false,
        }
    }
}

impl ChannelFrequency for TriangleState {
    fn get_period(self: &Self) -> u16 {
        self.period
    }
    fn get_period_min(self: &Self) -> u16 {
        1
    }
}

impl ChannelState for TriangleState {
    type Delta = TriangleDelta;

    fn transform(self: Self, delta: TriangleDelta) -> Self {
        match delta {
            TriangleDelta::SetPeriod(p) => Self { period: p, ..self },
            TriangleDelta::SetControlFlag(c) => Self {
                control_flag: c,
                ..self
            },
        }
    }

    fn signal_at(self: &Self, config: &ChannelTuning) -> f32 {
        if !self.control_flag {
            return 0.0;
        }

        let frequency = match self.get_frequency() {
            None => return 0.0,
            Some(f) => f,
        };

        let sample_offset = config.sample * (config.sample_rate as u64);
        let period_offset = (sample_offset % frequency as u64) as f32 / frequency;
        let signal = (0.25 - (period_offset - 0.5).abs()) * 4.0;
        return signal;
    }
}

////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct NoiseState {
    volume: u8,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum NoiseDelta {
    SetVolume(u8),
}

impl ChannelAmplitude for NoiseState {
    fn get_volume(self: &Self) -> u8 {
        return self.volume;
    }
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
        match self.get_amplitude() {
            None => 0.0,
            Some(max_amplitude) => {
                let mut random = thread_rng();
                random.gen_range(-max_amplitude, max_amplitude)
            }
        }
    }
}
