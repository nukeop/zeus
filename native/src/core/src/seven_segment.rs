pub struct Digit {
    pub val: u8
}

impl Digit {
    pub fn new(v: u8) -> Digit {
        Digit {
            val: v
        }
    }

    pub fn set_val(&mut self, v: u8) {
        self.val = v;
    }
}

pub struct SevenSegment {
    pub digits: Vec<Digit>
}

impl SevenSegment {
    pub fn new(d: u8) -> SevenSegment {
        let mut digits = Vec::new();
        for _ in 0..d {
            digits.push(Digit::new(0));
        }
        
        SevenSegment{digits}
    }

    pub fn set_digit(&mut self, d: usize, val: u8) {
        self.digits[d].val = val;
    }
}
