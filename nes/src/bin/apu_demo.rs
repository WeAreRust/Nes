extern crate bitflags;
extern crate nes;
extern crate sdl2;

use nes::apu::channel::*;
use nes::io::audio::NesAudioProcess;
use sdl2::audio::AudioSpecDesired;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let audio_subsystem = sdl_context.audio().unwrap();
  let desired_spec = AudioSpecDesired {
    freq: Some(48000),
    channels: Some(1),
    samples: Some(800),
  };

  let (send, recv) = mpsc::channel();

  let device = audio_subsystem
    .open_playback(None, &desired_spec, |spec| {
      NesAudioProcess::new(recv, spec.freq as u32)
    })
    .unwrap();

  device.resume();
  _arp(&send);
  _random_tune(&send);
  _decend(&send);
}

fn _arp(send: &Sender<ApuChannelDelta>) {
  for note in 40..80 {
    for offset in [0, 4, 7, 4].iter() {
      pew_all(
        send,
        vec![
          ApuChannelDelta::Pulse1(PulseDelta::SetVolume(64)),
          ApuChannelDelta::Pulse1(PulseDelta::SetPeriod(from_note(note + *offset))),
          ApuChannelDelta::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty0)),
        ],
      );
      std::thread::sleep(std::time::Duration::from_millis(20));
    }
  }
}

fn _random_tune(send: &Sender<ApuChannelDelta>) {
  for note in 0..80 {
    pew_all(
      send,
      vec![
        ApuChannelDelta::Pulse1(PulseDelta::SetVolume(64)),
        ApuChannelDelta::Pulse1(PulseDelta::SetPeriod(from_note(note))),
        ApuChannelDelta::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty0)),
        ApuChannelDelta::Triangle(TriangleDelta::SetPeriod(from_note(note))),
        ApuChannelDelta::Triangle(TriangleDelta::SetControlFlag(true)),
      ],
    );
    std::thread::sleep(std::time::Duration::from_millis(200));
  }

  pew_all(
    send,
    vec![ApuChannelDelta::Triangle(TriangleDelta::SetControlFlag(
      false,
    ))],
  );
}

fn from_note(note: u16) -> u16 {
  let semitone = (2.0f32).powf(1.0 / 12.0);
  let ntsc_octave_base = 39375000.0 / (22.0 * 16.0 * 55.0);
  let rel_freq = (1 << (note / 12)) as f32 * semitone.powf((note % 12) as f32);
  return (ntsc_octave_base / rel_freq).round() as u16 - 1;
}

fn _decend(send: &Sender<ApuChannelDelta>) {
  let total_steps = 1u32 << 16;
  let start_step = 1u32 << 4;
  let _scale_volume = <u8>::max_value() as f32 / total_steps as f32;
  let scale_peroid = ((1 << 11) - 1) as f32 / total_steps as f32;

  pew_all(
    send,
    vec![
      ApuChannelDelta::Pulse1(PulseDelta::SetVolume(64)),
      ApuChannelDelta::Pulse2(PulseDelta::SetVolume(64)),
      ApuChannelDelta::Pulse1(PulseDelta::SetPulseWidth(PulseWidth::Duty0)),
      ApuChannelDelta::Pulse2(PulseDelta::SetPulseWidth(PulseWidth::Duty3)),
    ],
  );

  for step in start_step..total_steps {
    let f_step = step as f32;
    let f_step_left = (total_steps - step) as f32;

    let noise_volume = (32.0 * ((step % 64) as f32 / 64.0)) as u8;
    //  if step < (total_steps / 2) { f_step * scale_volume }
    //  else { f_step_left * scale_volume };

    let pulse_1_pitch = f_step * scale_peroid;
    let pulse_2_pitch = (f_step * 0.75) * scale_peroid;
    let triangle_pitch = f_step_left * scale_peroid;

    println!(
      "p1 {} p2 {} nv {}",
      pulse_1_pitch, pulse_2_pitch, noise_volume
    );

    pew_all(
      send,
      vec![
        ApuChannelDelta::Noise(NoiseDelta::SetVolume(noise_volume as u8)),
        ApuChannelDelta::Pulse1(PulseDelta::SetPeriod(pulse_1_pitch as u16)),
        ApuChannelDelta::Pulse2(PulseDelta::SetPeriod(pulse_2_pitch as u16)),
        ApuChannelDelta::Triangle(TriangleDelta::SetPeriod(triangle_pitch as u16)),
      ],
    );
    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}

fn pew_all(send: &Sender<ApuChannelDelta>, deltas: Vec<ApuChannelDelta>) {
  send.send(ApuChannelDelta::Many(deltas)).unwrap();
}
