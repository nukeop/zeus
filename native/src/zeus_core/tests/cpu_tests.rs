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

    #[test]
    fn rand_test() {
        let mut cpu = CPU::new();
        cpu.regs.pc = 0x003D;

        cpu.ram.mem[0x003D] = 0x01;
        cpu.ram.mem[0x003E] = 0x05;
        cpu.ram.mem[0x003F] = 0x41;
        cpu.ram.mem[0x0040] = 0x00;
        cpu.rand();

        assert!(cpu.ram.mem[0x0041] != 0);
        assert!(cpu.ram.mem[0x0041] >= 1);
        assert!(cpu.ram.mem[0x0041] <= 5);
    }
}
