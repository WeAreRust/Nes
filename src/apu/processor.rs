// use apu::channel::{PulseDelta, TriangleDelta, NoiseDelta};
use memory::Memory;
use std::sync::mpsc::{Sender};

const APU_REGISTER_START: usize = 0x4000;
const APU_REGISTER_RANGE: usize = 22;

pub struct APU {
    previous_snapshot: APURegisterSnapshot,
    delta_stream: Sender<channel::ApuChannelDelta>,
}

pub struct APURegisterSnapshot {
    /// Includes registers $4000-$4015
    registers: [u8, APU_REGISTER_RANGE],
}

pub impl APURegisterSnapshot {
    fn create_from_memory(memory: Memory) -> Self {
        let registers = [0, APU_REGISTER_RANGE];
        for offset in 0..APU_REGISTER_RANGE {
            let read_index = (APU_REGISTER_START + offset) as u16;
            let read_value = memory.fetch(read_index);
            registers[offset] = read_value;
        }
        APURegisterSnapshot { registers }
    }
}

pub impl APU {
    fn create(sender: Sender<channel::ApuChannelDelta>) -> &Self {
        APU { delta_stream: sender }
    }
}

