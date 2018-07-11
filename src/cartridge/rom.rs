pub struct Rom {
    content: Vec<u8>,
}

impl Rom {
    pub fn new(content: Vec<u8>) -> Self {
        Rom {
            content: content,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        *self.content.get(addr as usize).unwrap()
    }
}