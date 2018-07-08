use sdl2::audio::{AudioCallback};
use std::sync::mpsc::{Receiver};

use apu::channel;
use apu::channel::{ChannelState};

pub struct NesAudioProcess {
    tick: u64,
    base_frequency: u32,
    channels: channel::ApuChannelState,
    delta_stream: Receiver<channel::ApuChannelDelta>,
}

impl AudioCallback for NesAudioProcess {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.apply_transforms();

        for elem in out.iter_mut() {
            let config = channel::ChannelTuning {
                tick: self.tick,
                base_frequency: self.base_frequency,
            };
            *elem = self.channels.signal_at(&config);
            self.tick += 1;
        }
    }
}

impl NesAudioProcess {
    pub fn new(delta_stream: Receiver<channel::ApuChannelDelta>, playback_freq: u32) -> Self {
        NesAudioProcess {
            tick: 0,
            channels: channel::ApuChannelState::default(),
            base_frequency: playback_freq,
            delta_stream: delta_stream,
        }
    }

    fn apply_transforms(self: &mut Self) {
        let channels = self.channels.clone();
        self.channels = self.delta_stream
            .try_iter()
            .fold(channels, |acc, delta| acc.transform(delta));
    }
}
