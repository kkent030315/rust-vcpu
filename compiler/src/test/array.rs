use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

fn generate_array_source(array: &[u8]) -> String {
    let mut s = String::from("array:\n");

    for c in array {
        s += &format!("db {c}\n");
    }

    s
}

fn test_array(array: &[u8]) {
    let array_src = generate_array_source(array);
    let s = format!("{}\n{array_src}", include_str!("array.S"));

    let mut builder = Builder::new();
    build_bytecode_s(s, &mut builder).unwrap();
    builder.finalize().unwrap();

    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);

    let array_loc = *builder.labels.get("array").unwrap();
    assert_ne!(array_loc, 0);

    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();

    assert_eq!(emulator.regs.read(Register::R0), 1u64);
    assert_eq!(emulator.regs.read(Register::R1), 2u64);
    assert_eq!(emulator.regs.read(Register::R2), 3u64);
    assert_eq!(emulator.regs.read(Register::R3), 4u64);
    assert_eq!(emulator.regs.read(Register::R4), 5u64);

    assert_ne!(emulator.ip() as usize, 0);
}

#[test]
fn array() {
    test_array(&[1, 2, 3, 4, 5]);
}
