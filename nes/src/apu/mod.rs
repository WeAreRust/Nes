use memory::{WriteAddr};

pub mod channel;
pub mod channel_differ;
pub mod processor;

pub trait Apu: WriteAddr {}
