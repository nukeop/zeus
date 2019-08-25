extern crate zeus_core;

#[cfg(test)]
mod cpu_tests {
    use zeus_core::cpu::CPU;

    #[test]
    fn reset_test() {
        let mut cpu = CPU::new();
        cpu.reset();

        assert_eq!(cpu.regs.x, 0);
        assert_eq!(cpu.regs.y, 0);
        assert_eq!(cpu.regs.t, 0);
        assert_eq!(cpu.regs.s, 0);
        assert_eq!(cpu.regs.pc, 0x2000);
        assert_eq!(cpu.regs.n, 0);

        for (i, byte) in cpu.ram.mem.iter().enumerate() {
            assert_eq!(*byte, 0);
        }
    }
}
