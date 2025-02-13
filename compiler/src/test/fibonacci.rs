use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

fn test_fibonacci(input: u64, output: u64) {
    let s = format!("mov r1, {}\n{}", input, include_str!("fibonacci.S"));
    let mut builder = Builder::new();
    build_bytecode_s(s, &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R0), output);
    assert_ne!(emulator.ip() as usize, 0);
}

#[test]
fn fibonacci() {
    test_fibonacci(0, 0);
    test_fibonacci(1, 1);
    test_fibonacci(2, 1);
    test_fibonacci(5, 5);
    test_fibonacci(10, 55);
    test_fibonacci(20, 6765);
    test_fibonacci(30, 832040);
    test_fibonacci(40, 102334155);
    test_fibonacci(50, 12586269025);
    test_fibonacci(93, 12200160415121876738);
}
