use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn jle() {
    {
        const S: &str = "
mov r0, 1
mov r1, 3
cmp r0, r1
jle less_or_equal
mov r0, 3
exit
less_or_equal:
mov r0, 7
exit
";
        let mut builder = Builder::new();
        build_bytecode_s(S, &mut builder).unwrap();
        builder.finalize().unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.execute().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 7u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 0);
        assert_eq!(rf.read_sf(), 1);
        assert_eq!(rf.read_of(), 0);
    }
    {
        const S: &str = "
mov r0, 9
mov r1, 3
cmp r0, r1
jle less_or_equal
mov r0, 3
exit
less_or_equal:
mov r0, 7
exit
";
        let mut builder = Builder::new();
        build_bytecode_s(S, &mut builder).unwrap();
        builder.finalize().unwrap();
        let dump = builder.dump().unwrap();
        let bytecode_len = dump.len();
        assert_ne!(bytecode_len, 0);
        let mut emulator = Emulator::with_bytecode(dump);
        emulator.execute().unwrap();
        assert_eq!(emulator.regs.read(Register::R0), 3u64);
        assert_ne!(emulator.ip() as usize, 0);
        let rf = emulator.regs.read_rf();
        assert_eq!(rf.read_zf(), 0);
        assert_eq!(rf.read_sf(), 0);
        assert_eq!(rf.read_of(), 0);
    }
}
