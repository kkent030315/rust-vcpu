use vm::emulator::{Emulator, Register};

use crate::builder::{build_bytecode_s, Builder};

#[test]
fn mov_r_imm() {
    let mut builder = Builder::new();
    build_bytecode_s("mov r0, 1000h\nexit\n", &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R0), 0x1000u64);
    assert_ne!(emulator.ip() as usize, 0);
}

#[test]
fn mov_rr() {
    let mut builder = Builder::new();
    build_bytecode_s("mov r0, 1000h\nmov r1, r0\nexit\n", &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R0), 0x1000u64);
    assert_eq!(emulator.regs.read(Register::R1), 0x1000u64);
    assert_ne!(emulator.ip() as usize, 0);
}

#[test]
fn mov_r_rm() {
    const S: &str = "
mov r0, byte [r0]
exit
";
    let mut builder = Builder::new();
    build_bytecode_s(S, &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump.to_owned());
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R0), dump[0] as u64);
    assert_ne!(emulator.ip() as usize, 0);
}

#[test]
fn mov_rm_r() {
    const S: &str = "
dq 0CCCCCCCCCCCCCCCCh
xor r1, r1
xor r4, r4
mov r0, 5F621844867E03A6h
mov qword [r1+r4], r0
mov r2, qword [r1+r4]
exit
";
    let mut builder = Builder::new();
    build_bytecode_s(S, &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    assert_eq!(
        u64::from_le_bytes(dump[0..8].try_into().unwrap()),
        0xCCCCCCCCCCCCCCCCu64
    );
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);
    let mut emulator = Emulator::with_bytecode(dump.to_owned());
    emulator.set_ip(8);
    emulator.execute().unwrap();
    assert_eq!(emulator.regs.read(Register::R2), 0x5F621844867E03A6u64);
    assert_eq!(emulator.dram.read_u64le(0).unwrap(), 0x5F621844867E03A6u64);
    assert_ne!(emulator.ip() as usize, 0);
}
