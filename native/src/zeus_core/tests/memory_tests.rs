extern crate zeus_core;

#[cfg(test)]
mod memory_tests {
    use zeus_core::memory::{RAM, Memory};

    #[test]
    fn store_byte_test() {
        let mut ram = RAM::new();

        ram.store_byte(0x0100, 0xAB);
        assert_eq!(ram.mem[0x100], 0xAB);
    }

    #[test]
    #[should_panic]
    fn store_byte_address_out_of_range() {
        let mut ram = RAM::new();
        ram.store_byte(0x2100, 0xAB);
    }
}
