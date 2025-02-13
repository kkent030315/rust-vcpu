use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn imul() {
    let mut builder = Builder::new();
    build_bytecode_s("mov r0, 6\nmov r1, 3\nimul r0, r1\nexit\n", &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R0), 18u64);
    assert_eq!(emulator.regs.read(Register::R1), 3u64);
    assert_ne!(emulator.ip() as usize, 0);
}
