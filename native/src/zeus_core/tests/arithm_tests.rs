extern crate zeus_core;

#[cfg(test)]
mod arithm_tests {
    use zeus_core::arithm;

    #[test]
    fn to_digits_test() {
        let val = format!("{:X}", 0xDEAD);
        let digits = arithm::to_digits(val);
        assert_eq!(digits[0], 0xD);
        assert_eq!(digits[1], 0xE);
        assert_eq!(digits[2], 0xA);
        assert_eq!(digits[3], 0xD);
    }

    #[test]
    fn parse_as_hex_test() {
        let val = format!("{:X}", 0x1234);
        let parsed = arithm::parse_as_hex(val);
        assert_eq!(parsed, 0x1234);
    }

    #[test]
    fn swizzle_test() {
        let val = 0x4321;
        assert_eq!(arithm::swizzle(val, 0x4321), 0x1234);        
    }

    #[test]
    fn swizzle_test2() {
        let val = 0xA11A;
        assert_eq!(arithm::swizzle(val, 0x6543), 0x00A1);        
    }

    #[test]
    fn swizzle_test3() {
        let val = 0xBCD4;
        assert_eq!(arithm::swizzle(val, 0x2223), 0xCCCD);        
    }

    #[test]
    fn get_random_test() {
        let result = arithm::get_random(0, 10);
        assert!(result >= 0);
        assert!(result <= 10);
    }
}
