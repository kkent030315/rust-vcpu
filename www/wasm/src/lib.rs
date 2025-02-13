use std::sync::Mutex;

use compiler::builder::{build_bytecode_s, Builder};
use once_cell::sync::Lazy;
use vm::emulator::{Emulator, Register};
use wasm_bindgen::prelude::*;

static EMULATOR: Lazy<Mutex<Emulator>> = Lazy::new(|| Mutex::new(Emulator::new()));

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let mut builder = Builder::new();
    build_bytecode_s(source, &mut builder).map_err(|e| format!("{e:?}"))?;
    builder.finalize().map_err(|e| format!("{e:?}"))?;
    let dump = builder.dump().map_err(|e| format!("{e:?}"))?;
    Ok(dump)
}

#[wasm_bindgen]
pub fn init_vm(bytecode: &[u8]) {
    let mut emulator = EMULATOR.lock().unwrap();
    *emulator = Emulator::with_bytecode(bytecode);
}

#[wasm_bindgen]
pub fn read_reg(i: u8) -> u64 {
    let emulator = EMULATOR.lock().unwrap();
    emulator.regs.read(Register::from_repr(i as u8).unwrap())
}

#[wasm_bindgen]
pub fn get_cycle() -> u64 {
    let emulator = EMULATOR.lock().unwrap();
    emulator.cycle
}

#[wasm_bindgen]
pub fn execute() -> Result<(), String> {
    let mut emulator = EMULATOR.lock().unwrap();
    emulator.execute().map_err(|e| format!("{e:?}"))
}

#[wasm_bindgen]
pub fn single_step() -> Result<(), String> {
    let mut emulator = EMULATOR.lock().unwrap();
    emulator.single_step().map_err(|e| format!("{e:?}"))
}
