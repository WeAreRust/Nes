use sdl2::audio::{AudioCallback};
use std::sync::mpsc::{Receiver};

use apu::state;


type PulseCallback = ChannelCallback<state::PulseState, state::PulseDelta>;
type TriangleCallback = ChannelCallback<state::TriangleState, state::TriangleDelta>;
type PulseReceiver = Receiver<state::PulseDelta>;
type TriangleReceiver = Receiver<state::TriangleDelta>;

struct ChannelCallback<State, Delta> {
    receiver: Receiver<Delta>,
    state: State,
}

pub struct NesAudioProcess {
    tick: u64,
    pulse_1: PulseCallback,
    pulse_2: PulseCallback,
    triangle: TriangleCallback,
}

impl<S, D> ChannelCallback<S, D> where S: state::ChannelState<Delta = D> {
    fn new(receiver: Receiver<D>) -> ChannelCallback<S, D> {
        let state = S::initial_state();
        ChannelCallback { receiver, state }
    }

    fn apply_transforms(self: &mut Self) -> () {
        self.state = self.receiver.try_iter().fold(
            self.state.clone(),
            |acc, delta| acc.transform(delta),
        );
    }

    fn signal_at(self: &Self, tick: u64) -> f32 {
        self.state.signal_at(tick)
    }
}

impl AudioCallback for NesAudioProcess {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.pulse_1.apply_transforms();
        self.pulse_2.apply_transforms();
        self.triangle.apply_transforms();

        for elem in out.iter_mut() {
            *elem = 0f32;
            *elem += self.pulse_1.signal_at(self.tick);
            *elem += self.pulse_2.signal_at(self.tick);
            *elem += self.triangle.signal_at(self.tick);
            self.tick += 1;
        }
    }
}

impl NesAudioProcess {
    pub fn new(p1_recv: PulseReceiver, p2_recv: PulseReceiver, t_recv: TriangleReceiver) -> Self {
        NesAudioProcess {
            tick: 0,
            pulse_1: PulseCallback::new(p1_recv),
            pulse_2: PulseCallback::new(p2_recv),
            triangle: TriangleCallback::new(t_recv),
        }
    }
}
