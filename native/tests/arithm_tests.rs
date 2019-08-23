extern crate zeus;

#[cfg(test)]
mod arithm_tests {
    use zeus::core::arithm;

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
