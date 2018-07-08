#![feature(fixed_size_array)]

#[macro_use]
extern crate bitflags;

extern crate core;
extern crate rand;
extern crate sdl2;

pub mod io;
pub mod cpu;
pub mod apu;
pub mod memory;
