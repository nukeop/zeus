pub struct Digit {
    val: u8
}

impl Digit {
    pub fn new(v: u8) -> Digit {
        Digit {
            val: v
        }
    }
}

pub struct SevenSegment {
    digits: Vector<Digit>
}

impl SevenSegment {
    pub fn new(digits: u8) -> SevenSegment {
        let digits = Vec::new();
        for _ in 1..digits {
            digits.push(Digit::new(0));
        }
        SevenSegment{digits}
    }
}
