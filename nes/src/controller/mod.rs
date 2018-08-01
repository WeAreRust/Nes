use memory::{ReadAddr, WriteAddr};

pub mod joypad;

pub trait Controller: ReadAddr + WriteAddr {}
