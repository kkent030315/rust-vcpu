use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn test() {
    {
        let mut builder = Builder::new();
        build_bytecode_s("mov r0, 5\ntest r0, r0\n", &mut builder).unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.single_step().unwrap();
        emulator.single_step().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 5u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 0);
    }
    {
        let mut builder = Builder::new();
        build_bytecode_s("mov r0, 0\ntest r0, r0\n", &mut builder).unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.single_step().unwrap();
        emulator.single_step().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 0u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 1);
    }
}
