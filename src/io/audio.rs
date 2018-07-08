use sdl2::audio::{AudioCallback};
use std::fmt::Debug;
use std::sync::mpsc::{Receiver};

use apu::channel;


type PulseCallback = ChannelCallback<channel::PulseState, channel::PulseDelta>;
type TriangleCallback = ChannelCallback<channel::TriangleState, channel::TriangleDelta>;
type NoiseCallback = ChannelCallback<channel::NoiseState, channel::NoiseDelta>;
type PulseReceiver = Receiver<channel::PulseDelta>;
type TriangleReceiver = Receiver<channel::TriangleDelta>;
type NoiseReceiver = Receiver<channel::NoiseDelta>;

#[derive(Debug)]
struct ChannelCallback<S, D> {
    receiver: Receiver<D>,
    state: S,
}

pub struct NesAudioProcess {
    tick: u64,
    base_frequency: f32,
    pulse_1: PulseCallback,
    pulse_2: PulseCallback,
    triangle: TriangleCallback,
    noise: NoiseCallback,
}

impl<S, D> ChannelCallback<S, D> where S: channel::ChannelState<Delta = D>, D: Debug {
    fn new(receiver: Receiver<D>) -> ChannelCallback<S, D> {
        let state = S::default();
        ChannelCallback { receiver, state }
    }

    fn apply_transforms(self: &mut Self) -> () {
        let init_state = self.state.clone();
        self.state = self.receiver
            .try_iter()
            .fold(init_state, |acc, delta| {
                acc.transform(delta)
            });
    }

    fn signal(self: &Self, config: &channel::ChannelTuning) -> f32 {
        self.state.signal_at(config)
    }
}

impl AudioCallback for NesAudioProcess {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.pulse_1.apply_transforms();
        self.pulse_2.apply_transforms();
        self.triangle.apply_transforms();
        self.noise.apply_transforms();

        for elem in out.iter_mut() {
            let config = channel::ChannelTuning {
                tick: self.tick,
                base_frequency: self.base_frequency,
            };
            *elem = 0f32;
            *elem += self.pulse_1.signal(&config);
            *elem += self.pulse_2.signal(&config);
            *elem += self.triangle.signal(&config);
            *elem += self.noise.signal(&config);
            self.tick += 1;
        }
    }
}

impl NesAudioProcess {
    pub fn new(
            p1_recv: PulseReceiver,
            p2_recv: PulseReceiver,
            t_recv: TriangleReceiver,
            n_recv: NoiseReceiver,
    ) -> Self {
        NesAudioProcess {
            tick: 0,
            pulse_1: PulseCallback::new(p1_recv),
            pulse_2: PulseCallback::new(p2_recv),
            triangle: TriangleCallback::new(t_recv),
            noise: NoiseCallback::new(n_recv),
            base_frequency: 44100.0,
        }
    }
}
