use core::arithm::swizzle;
use core::memory::{RAM,Memory};

pub struct Registers {
    x: u8,
    y: u8,
    t: u8,
    pc: u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            x: 0,
            y: 0,
            t: 0,
            pc: 0x2000
        }
    }
}

pub struct CPU {
    pub regs: Registers,
    pub ram: RAM
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new(),
            ram: RAM::new()
        }
    }

    pub fn step(&mut self) {
        let next = self.load_byte_increment_pc();
        self.decode(next);
    }

    pub fn load_byte_increment_pc(&mut self) -> u8 {
        let pc = self.regs.pc;
        let val = self.ram.load_byte(pc);
        self.regs.pc += 1;
        val
    }

    pub fn get_addr(&mut self) -> u16 {
        let low = self.load_byte_increment_pc();
        let hi = self.load_byte_increment_pc();

        ((hi as u16) << 8) | (low as u16)
    }

    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.noop(),
            0x01 => self.mvix(),
            0x02 => self.mviy(),
            0x03 => self.mvit(),
            0x04 => self.mvax(),
            0x05 => self.mvay(),
            0x06 => self.mvat(),
            0x07 => self.mvxa(),
            0x08 => self.mvya(),
            0x09 => self.mvta(),
            0x0A => self.mvpa(),
            0x0B => self.generate_arithm(|x, y| {
                x.wrapping_add(y)
            }),
            0x0C => self.generate_arithm(|x, y| {
                x.wrapping_sub(y)
            }),
            0x0D => self.generate_arithm(|x, y| {
                x.wrapping_mul(y)
            }),
            0x0E => self.generate_arithm(|x, y| {
                x.wrapping_div(y)
            }),
            0x0F => self.generate_arithm(|x, y| {
                x.wrapping_rem(y)
            }),
            0x10 => self.swiz(),
            0x11 => self.generate_arithm(|x, y| {
                x & y
            }),
            0x12 => self.generate_arithm(|x, y| {
                x | y
            }),
            0x13 => self.generate_arithm(|x, y| {
                x ^ y
            }),
            _ => panic!("Unimplemented opcode: {:X}", opcode)
        };
    }

    pub fn noop(&mut self) {}
    pub fn mvix(&mut self) {
        let pc = self.regs.pc;
        let val = self.load_byte_increment_pc();
        self.regs.x = val;
    }

    pub fn mviy(&mut self) {
        let pc = self.regs.pc;
        let val = self.load_byte_increment_pc();
        self.regs.y = val;
    }

    pub fn mvit(&mut self) {
        let pc = self.regs.pc;
        let val = self.load_byte_increment_pc();
        self.regs.t = val;
    }

    pub fn mvax(&mut self) {
        let addr = self.get_addr();
        let byte = self.ram.load_byte(addr);
        self.regs.x = byte;        
    }

    pub fn mvay(&mut self) {
        let addr = self.get_addr();
        let byte = self.ram.load_byte(addr);
        self.regs.y = byte;        
    }

    pub fn mvat(&mut self) {
        let addr = self.get_addr();
        let byte = self.ram.load_byte(addr);
        self.regs.t = byte;        
    }

    pub fn mvxa(&mut self) {
        let addr = self.get_addr();
        if (addr < 0x2000) {
            self.ram.store_byte(addr, self.regs.x);
        }
    }

    pub fn mvya(&mut self) {
        let addr = self.get_addr();
        if (addr < 0x2000) {
            self.ram.store_byte(addr, self.regs.y);
        }
    }
    
    pub fn mvta(&mut self) {
        let addr = self.get_addr();
        if (addr < 0x2000) {
            self.ram.store_byte(addr, self.regs.t);
        }
    }
    
    pub fn mvpa(&mut self) {
        let addr = self.get_addr();
        if (addr < 0x2000) {
            self.ram.store_word(addr, self.regs.pc);
        }
    }

    pub fn generate_arithm<F>(&mut self, closure: F) where F: Fn(u8, u8) -> u8 {
        let addr1 = self.get_addr();
        let addr2 = self.get_addr();
        let addr3 = self.get_addr();

        if (addr3 < 0x2000) {
            let result = closure(
                self.ram.load_byte(addr1),
                self.ram.load_byte(addr2)
            );
            self.ram.store_byte(addr3, result);
        }
    }

    pub fn swiz(&mut self) {
        let addr1 = self.get_addr();
        let addr2 = self.get_addr();
        let addr3 = self.get_addr();

        if (addr3 < 0x2000) {
            let result = swizzle(
                self.ram.load_word(addr1),
                self.ram.load_word(addr2)
            );
            self.ram.store_word(addr3, result);
        }
    }
}
