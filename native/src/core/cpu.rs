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
    regs: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new()
        }
    }
}
