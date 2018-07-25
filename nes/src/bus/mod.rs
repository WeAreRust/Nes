use cartridge::Cartridge;
use memory::{ReadAddr, WriteAddr};
use memory::block::BlockMemory;

pub struct Bus<'a> {
    cartridge: &'a mut Cartridge,
    ram: &'a mut BlockMemory,
}

impl <'a> Bus<'a> {
    pub fn new(cartridge: &'a mut Cartridge, ram: &'a mut BlockMemory) -> Self {
        Bus{
            cartridge: cartridge,
            ram: ram,
        }
    }
}

impl <'a> ReadAddr for Bus<'a> {
    fn read_addr(&self, addr: u16) -> u8 {
        match addr {
            // RAM
            0x0000...0x1FFF => self.ram.read_addr(addr & 0x07FF),
            // Catridge ROM
            0x8000...0xFFFF => self.cartridge.mapper.read_addr(addr),
            _ => panic!("Bus addr not implemented for ${:04X}", addr),
        }
    }
}

impl <'a> WriteAddr for Bus<'a> {
    fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
        match addr {
            // RAM
            0x0000...0x1FFF => self.ram.write_addr(addr & 0x07FF, value),
            // Catridge ROM
            0x8000...0xFFFF => self.cartridge.mapper.write_addr(addr, value),
            _ => panic!("Bus addr not implemented for ${:04X}", addr),
        }
    }
}