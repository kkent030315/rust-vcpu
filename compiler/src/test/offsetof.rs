use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn offsetof() {
    const S: &str = "
mov r0, offsetof data
exit

data:
    db 82
";
    let mut builder = Builder::new();
    build_bytecode_s(S, &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_eq!(
        emulator.regs.read(Register::R0),
        *builder.labels.get("data").unwrap()
    );
    assert_ne!(emulator.ip() as usize, 0);
}
