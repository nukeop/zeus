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
    pub mem: [u8; 0x2000],
    pub rom_mem: [u8; 0xE000]
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            mem: [0; 0x2000],
            rom_mem: [0; 0xE000]
        }
    }

    pub fn clear(&mut self) {
        self.mem = [0; 0x2000];
    }
}

impl Memory for RAM {
    fn load_byte(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.mem[addr as usize & 0x1fff]
        } else {
            self.rom_mem[(addr - 0x2000) as usize]
        }
    }

    fn store_byte(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.mem[addr as usize & 0x1fff] = val;
        } else {
            panic!("Can't write to ROM");
        }
    }
}
