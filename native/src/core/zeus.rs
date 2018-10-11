use core::cpu::CPU;
use core::memory::RAM;

pub struct Zeus {
    cpu: CPU,
    ram: RAM
}

impl Zeus {
    pub fn new() -> Zeus {
        Zeus {
            cpu: CPU::new(),
            ram: RAM::new()
        }
    }
}
