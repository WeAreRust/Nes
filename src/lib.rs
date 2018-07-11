#![feature(fixed_size_array)]

#[cfg(test)]
extern crate asm6502;

#[macro_use]
extern crate bitflags;

extern crate core;
extern crate rand;
extern crate sdl2;

pub mod io;
pub mod cpu;
pub mod apu;
pub mod memory;
pub mod cartridge;
