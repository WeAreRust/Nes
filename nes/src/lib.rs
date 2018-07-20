#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(nll)]

#[cfg(test)]
extern crate asm6502;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate nes_derives;

extern crate bytes;
extern crate core;
extern crate rand;
extern crate sdl2;

pub mod apu;
pub mod cartridge;
pub mod clock;
pub mod cpu;
pub mod io;
pub mod memory;
pub mod ppu;
