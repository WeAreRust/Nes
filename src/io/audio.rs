use sdl2::audio::AudioCallback;
use std::sync::mpsc::Receiver;

use apu::channel;
use apu::channel::ChannelState;

pub struct NesAudioProcess {
    sample: u64,
    sample_rate: u32,
    channels: channel::ApuChannelState,
    delta_stream: Receiver<channel::ApuChannelDelta>,
}

impl AudioCallback for NesAudioProcess {
    type Channel = f32;

    fn callback(&mut self, samples: &mut [Self::Channel]) {
        self.apply_transforms();

        for elem in samples.iter_mut() {
            let config = channel::ChannelTuning {
                sample: self.sample,
                sample_rate: self.sample_rate,
            };
            *elem = self.channels.signal_at(&config);
            self.sample += 1;
        }
    }
}

impl NesAudioProcess {
    pub fn new(delta_stream: Receiver<channel::ApuChannelDelta>, playback_freq: u32) -> Self {
        NesAudioProcess {
            sample: 0,
            channels: channel::ApuChannelState::default(),
            sample_rate: playback_freq,
            delta_stream: delta_stream,
        }
    }

    fn apply_transforms(self: &mut Self) {
        let channels = self.channels.clone();
        self.channels = self
            .delta_stream
            .try_iter()
            .fold(channels, |acc, delta| acc.transform(delta));
    }
}
