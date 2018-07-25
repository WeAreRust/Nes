use cartridge::Cartridge;
use memory::{ReadAddr, WriteAddr};

pub struct Bus<'a> {
    cartridge: &'a mut Cartridge
}

impl <'a> Bus<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> Self {
        Bus{
            cartridge: cartridge,
        }
    }
}

impl <'a> ReadAddr for Bus<'a> {
    fn read_addr(&self, addr: u16) -> u8 {
        match addr {
            0x8000...0xFFFF => self.cartridge.mapper.read_addr(addr),
            _ => panic!("Bus addr not implemented for ${:04X}", addr),
        }
    }
}

impl <'a> WriteAddr for Bus<'a> {
    fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
        match addr {
            0x8000...0xFFFF => self.cartridge.mapper.write_addr(addr, value),
            _ => panic!("Bus addr not implemented for ${:04X}", addr),
        }
    }
}