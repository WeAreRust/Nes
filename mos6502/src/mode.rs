/// Marker trait
pub trait ImpliedMode {}

pub trait ValueMode {
  fn read(&self, core: &mut Core, memory: &mut Memory) -> u8;
  fn write(&self, core: &mut Core, memory: &mut Memory, value: u8);
}

pub trait AddressMode {
  fn get_address(&self, core: &mut Core, memory: &mut Memory) -> u16;
}

impl ValueMode for AddressMode {
  fn read(&self, core: &mut Core, memory: &mut Memory) -> u8 {
    let address = self.get_address(core, memory);
    memory.read(address)
  }

  fn write(&self, core: &mut Core, memory: &mut Memory, value: u8) {
    let address = self.get_address(core, memory);
    memory.write(address, value);
  }
}

pub struct Accumulator;
impl ValueMode for Accumulator {
  fn read(&self, core: &mut Core, _memory: &mut Memory) -> u8 {
    core.acc
  }

  fn write(&self, core: &mut Core, _memory: &mut Memory, value: u8) {
    core.acc = value;
  }
}

pub struct Absolute;
impl AddressMode for Absolute {
  fn get_address(&self, _core: &mut Core, _memory: &mut Memory) -> u16 {
    unimplemented!();
  }
}

// TODO: Can this be derived?
impl ValueMode for Absolute {
  fn read(&self, core: &mut Core, memory: &mut Memory) -> u8 {
    let address = self.get_address(core, memory);
    memory.read(address)
  }

  fn write(&self, core: &mut Core, memory: &mut Memory, value: u8) {
    let address = self.get_address(core, memory);
    memory.write(address, value);
  }
}
