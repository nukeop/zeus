pub trait Memory {
    fn load_byte(&mut self, addr: u16) -> u8;
    fn store_byte(&mut self, addr: u16, val: u8);
    fn load_word(&mut self, addr: u16) -> u16 {
        self.load_byte(addr) as u16 | (self.load_byte(addr + 1) as u16) << 8
    }
    fn store_word(&mut self, addr: u16, val: u16) {      
        self.store_byte(addr, (val & 0xff) as u8);
        self.store_byte(addr + 1, ((val >> 8) & 0xff) as u8);
    }
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
