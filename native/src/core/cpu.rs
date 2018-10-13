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

    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.noop(),
            0x01 => self.mvix(),
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
}
