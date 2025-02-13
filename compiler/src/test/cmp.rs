use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn cmp() {
    {
        let mut builder = Builder::new();
        build_bytecode_s("mov r0, 0\nmov r1, 5\ncmp r0, r1\nexit\n", &mut builder).unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.execute().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 0u64);
        assert_eq!(emulator.regs.read(Register::R1), 5u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 0);
    }
    {
        let mut builder = Builder::new();
        build_bytecode_s("mov r0, 0\nmov r1, 0\ncmp r0, r1\nexit\n", &mut builder).unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.execute().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 0u64);
        assert_eq!(emulator.regs.read(Register::R1), 0u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 1);
    }
}
