pub trait Memory {
    fn load_byte(&mut self, addr: u16) -> u8;
    fn store_byte(&mut self, addr: u16, val: u8);
}

pub struct RAM {
    pub mem: [u8; 0x2000]
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            mem: [0; 0x2000]
        }
    }
}

impl Memory for RAM {
    fn load_byte(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize & 0x1fff]
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize & 0x1fff] = val;
    }
}
