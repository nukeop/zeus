use rand::Rng;

pub fn to_digits(s: String) -> Vec<u32> {
    s.chars().map(|x| x.to_digit(16).unwrap()).collect()
}

pub fn parse_as_hex(s: String) -> u16 {
    let digits: Vec<u16> = s.chars().map(|x| x.to_digit(16).unwrap() as u16).collect();

    digits[0] * 4096 + digits[1] * 256 + digits[2] * 16 + digits[3]
}

pub fn swizzle(val: u16, mask: u16) -> u16 {
    let val_s = format!("{:X}", val);
    let val_digits = to_digits(val_s);
    let mask_s = format!("{:X}", mask);
    let mask_digits: Vec<u32> = to_digits(mask_s);
    let mask_vec: Vec<u32> = mask_digits.iter().map(|x| if x>&((4 as u32)) {5 as u32} else {*x-1}).collect();
    
    let result: Vec<String> = mask_vec.iter().map(|x|
                                                  (if x>&((4 as u32)) {"0".to_string()} else {format!("{:X}", val_digits[*x as usize])})
    ).collect();
    parse_as_hex(result.join(""))
}

pub fn get_random(lower: u8, upper: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower, upper+1)
}
