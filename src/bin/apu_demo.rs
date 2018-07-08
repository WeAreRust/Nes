#[macro_use]
extern crate bitflags;
extern crate sdl2;
extern crate nes;

use sdl2::audio::{AudioSpecDesired};
use std::sync::mpsc;
use nes::apu::channel::{NoiseDelta};
use nes::io::audio::NesAudioProcess;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let (_, p1_recv) = mpsc::channel();
    let (_, p2_recv) = mpsc::channel();
    let (_, t_recv) = mpsc::channel();
    let (n_send, n_recv) = mpsc::channel();

    let device = audio_subsystem.open_playback(None, &desired_spec, |_| {
        NesAudioProcess::new(p1_recv, p2_recv, t_recv, n_recv)
    }).unwrap();

    device.resume();
    n_send.send(NoiseDelta::SetVolume(64)).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
