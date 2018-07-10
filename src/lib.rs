#![feature(fixed_size_array)]

#[cfg(test)]
extern crate asm6502;

#[macro_use]
extern crate bitflags;

extern crate core;

pub mod cpu;
pub mod memory;
pub mod file;
