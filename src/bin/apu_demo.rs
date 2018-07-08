#[macro_use]
extern crate bitflags;
extern crate sdl2;

mod apu;
mod cpu;
mod io;

use sdl2::audio::{AudioSpecDesired};
use std::sync::mpsc;
use apu::state::{PulseDelta};
use io::audio::NesAudioProcess;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let (p1_send, p1_recv) = mpsc::channel();
    let (_, p2_recv) = mpsc::channel();
    let (_, t_recv) = mpsc::channel();

    let device = audio_subsystem.open_playback(None, &desired_spec, |_| {
        NesAudioProcess::new(p1_recv, p2_recv, t_recv)
    }).unwrap();

    device.resume();
    p1_send.send(PulseDelta::SetVolume(32)).unwrap();
    p1_send.send(PulseDelta::PlayNote(32)).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
