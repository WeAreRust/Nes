#![feature(fixed_size_array)]
#![allow(dead_code)]

#[cfg(test)]
extern crate asm6502;

#[macro_use]
extern crate bitflags;

extern crate bytes;
extern crate core;
extern crate rand;
extern crate sdl2;

pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod io;
pub mod memory;
pub mod ppu;
