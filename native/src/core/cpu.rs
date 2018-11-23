use core::arithm::swizzle;
use core::memory::{RAM,Memory};
use core::rom::Rom;
use core::screen::ScreenDriver;
use core::seven_segment::SevenSegment;

pub struct Registers {
    x: u8,
    y: u8,
    t: u8,
    s: u8,
    pc: u16,
    n: u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            x: 0,
            y: 0,
            t: 0,
            s: 0,
            pc: 0x2000,
            n: 0
        }
    }
}

pub struct CPU {
    pub regs: Registers,
    pub ram: RAM,
    pub screen: ScreenDriver,
    pub score: SevenSegment,
    pub hi_score: SevenSegment,
    pub speed_lcd: SevenSegment,
    pub level_lcd: SevenSegment
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new(),
            ram: RAM::new(),
            screen: ScreenDriver::new(16, 20),
            score: SevenSegment::new(5),
            hi_score: SevenSegment::new(5),
            speed_lcd: SevenSegment::new(2),
            level_lcd: SevenSegment::new(2)
        }
    }

    pub fn run_frame(&mut self) {
        loop {
            self.step();
            if(self.regs.n == 0) {
                break;
            }
        }
    }
    
    pub fn step(&mut self) {
        if (self.regs.n == 0xFFFF) {
            self.sync();
        } else {
            let next = self.load_byte_increment_pc();
            if (next != 0) {
                println!("Opcode: 0x{:X}", next);
            }
            
            self.decode(next);
        }

        self.regs.n = self.regs.n.wrapping_add(1);
    }

    pub fn sync(&mut self) {
        let screen_mem = self.ram.mem
            .iter()
            .take(40);
        for (i, byte) in screen_mem.enumerate() {
            self.screen.pixels[i] = *byte;
        }

        let score_mem = self.ram.mem.iter().skip(41).take(5).enumerate();
        for (i, byte) in score_mem {
            self.score.set_digit(i, *byte);
        }

        let hi_score_mem = self.ram.mem.iter().skip(46).take(5).enumerate();
        for (i, byte) in hi_score_mem {
            self.hi_score.set_digit(i, *byte);
        }

        let speed_lcd_mem = self.ram.mem.iter().skip(51).take(2).enumerate();
        for (i, byte) in speed_lcd_mem {
            self.speed_lcd.set_digit(i, *byte);
        }

        let level_lcd_mem = self.ram.mem.iter().skip(53).take(2).enumerate();
        for (i, byte) in level_lcd_mem {
            self.level_lcd.set_digit(i, *byte);
        }
    }

    pub fn reset(&mut self) {
        self.regs = Registers::new();
        self.ram.clear();
        self.sync();
    }

    pub fn load_byte_increment_pc(&mut self) -> u8 {
        let pc = self.regs.pc;
        let val = self.ram.load_byte(pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        val
    }

    pub fn load_word_increment_pc(&mut self) -> u16 {
        let pc = self.regs.pc;
        let val = self.ram.load_word(pc);
        self.regs.pc = self.regs.pc.wrapping_add(2);
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
            0x0B => self.addx(),
            0x0C => self.addy(),
            0x0D => self.addt(),
            0x0E => self.subx(),
            0x0F => self.suby(),
            0x10 => self.subt(),
            0x11 => self.copy(),
            0x12 => self.cpid(),
            0x13 => self.cpir(),
            0x14 => self.generate_binary(|x, y| x.wrapping_add(y)),
            0x15 => self.generate_binary(|x, y| x.wrapping_sub(y)),
            0x16 => self.generate_binary(|x, y| x.wrapping_mul(y)),
            0x17 => self.generate_binary(|x, y| x.wrapping_div(y)),
            0x18 => self.generate_binary(|x, y| x.wrapping_rem(y)),
            0x19 => self.swiz(),
            0x1A => self.generate_binary(|x, y| x & y),
            0x1B => self.generate_binary(|x, y| x | y),
            0x1C => self.generate_binary(|x, y| x ^ y),
            0x1D => self.generate_unary(|x| !x),
            0x1E => self.generate_unary(|x| x << 1),
            0x1F => self.generate_unary(|x| x >> 1),
            0x20 => self.generate_cmp(|x, y| x == y),
            0x21 => self.generate_cmp(|x, y| x > y),
            0x22 => self.generate_cmp(|x, y| x < y),
            0x23 => self.jump(),
            0x24 => self.tjmp(),
            0x25 => self.fjmp(),
            0x26 => self.rjmp(),
            0x27 => self.ijmp(),
            0x28 => self.bank(),
            0x29 => self.rand(),
            0x2A => self.wait(),
            0x2B => self.clrs(),
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

    pub fn generate_binary<F>(&mut self, closure: F) where F: Fn(u8, u8) -> u8 {
        let addr1 = self.get_addr();
        let addr2 = self.get_addr();
        let dest = self.get_addr();

        if (dest < 0x2000) {
            let result = closure(
                self.ram.load_byte(addr1),
                self.ram.load_byte(addr2)
            );
            self.ram.store_byte(dest, result);
        }
    }

    pub fn swiz(&mut self) {
        let addr1 = self.get_addr();
        let addr2 = self.get_addr();
        let dest = self.get_addr();

        if (dest < 0x2000) {
            let result = swizzle(
                self.ram.load_word(addr1),
                self.ram.load_word(addr2)
            );
            self.ram.store_word(dest, result);
        }
    }

    pub fn addx(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.x = self.regs.x.wrapping_add(val);
    }

    pub fn addy(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.y = self.regs.y.wrapping_add(val);
    }

    pub fn addt(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.t = self.regs.t.wrapping_add(val);
    }

    pub fn subx(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.x = self.regs.x.wrapping_sub(val);
    }

    pub fn suby(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.y = self.regs.y.wrapping_sub(val);
    }

    pub fn subt(&mut self) {
        let val = self.load_byte_increment_pc();
        self.regs.t = self.regs.t.wrapping_sub(val);
    }

    pub fn copy(&mut self) {
        let val = self.load_byte_increment_pc();
        let dest = self.get_addr();

        if(dest < 0x2000) {
            self.ram.store_byte(dest, val);
        }
    }

    pub fn cpid(&mut self) {
        let src = self.get_addr();
        let dest = self.get_addr() + self.regs.y as u16;

        let val = self.ram.load_byte(src);

        if(dest < 0x2000) {
            self.ram.store_byte(dest, val);
        }
    }

    pub fn cpir(&mut self) {
        let src = self.get_addr() + self.regs.y as u16;
        let dest = self.get_addr();
        let val = self.ram.load_byte(src);

        if(dest < 0x2000) {
            self.ram.store_byte(dest, val);
        }
    }

    pub fn generate_unary<F>(&mut self, closure: F) where F: Fn(u8) -> u8 {
        let addr = self.get_addr();
        let dest = self.get_addr();

        if(dest < 0x2000) {
            let result = closure(self.ram.load_byte(addr));
            self.ram.store_byte(dest, result);
        }
    }

    pub fn generate_cmp<F>(&mut self, closure: F) where F: Fn(u8, u8) -> bool {
        let addr1 = self.get_addr();
        let addr2 = self.get_addr();
        if (closure(self.ram.load_byte(addr1), self.ram.load_byte(addr2))) {
            self.regs.t = 1;
        }
    }

    pub fn jump(&mut self) {
        let val = self.load_word_increment_pc();
        self.regs.pc = val;
    }

    pub fn tjmp(&mut self) {
        if (self.regs.t != 0) {
            self.jump();
        } else {
            self.regs.pc = self.regs.pc.wrapping_add(2);
        }
    }

    pub fn fjmp(&mut self) {
        if (self.regs.t == 0) {
            self.jump();
        } else {
            self.regs.pc = self.regs.pc.wrapping_add(2);
        }
    }

    pub fn rjmp(&mut self) {
        panic!("Not implemented yet");
    }

    pub fn ijmp(&mut self) {
        let addr = self.get_addr();
        let jump_dest = self.ram.load_word(addr);
        self.regs.pc = jump_dest;
    }

    pub fn bank(&mut self) {
        panic!("Not implemented yet");
    }

    pub fn rand(&mut self) {
        panic!("Not implemented yet");
    }

    pub fn wait(&mut self) {
        self.regs.n = 0xFFFE;
    }

    pub fn clrs(&mut self) {
        for i in 1..40 {
            self.ram.mem[i] = 0;
        }
    }
}
