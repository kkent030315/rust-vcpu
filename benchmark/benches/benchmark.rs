use std::collections::HashMap;

use compiler::builder::{build_bytecode_s, Builder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vm::emulator::Emulator;

fn generate_source<S: AsRef<str>>(label: S, arr: &[u8]) -> String {
    let mut s = String::from(format!("{}:\n", label.as_ref()));

    for c in arr {
        s += &format!("db {c}\n");
    }

    s
}

fn compile_fibonacci_bytecode(n: u64) -> Vec<u8> {
    let s = format!(
        "mov r1, {n}\n{}",
        include_str!("../../compiler/src/test/fibonacci.S")
    );
    let mut builder = Builder::new();
    build_bytecode_s(s, &mut builder).unwrap();
    builder.finalize().unwrap();
    builder.dump().unwrap()
}

fn compile_rc4_bytecode(key: &[u8], buf_len: usize) -> Vec<u8> {
    let buf = vec![0u8; buf_len];
    let key_src = generate_source("key", key);
    let buf_src = generate_source("buf", &buf);
    let s = format!(
        "{}\n{key_src}\n{buf_src}\n",
        include_str!("../../compiler/src/test/rc4.S")
            .replace("%KEYLEN%", &format!("{}", key.len()))
            .replace("%BUFLEN%", &format!("{}", buf.len()))
    );
    let mut builder = Builder::new();
    build_bytecode_s(s, &mut builder).unwrap();
    builder.finalize().unwrap();
    builder.dump().unwrap()
}

fn test_fibonacci(bytecode: &[u8]) {
    let mut emulator = Emulator::with_bytecode(bytecode);
    emulator.execute().unwrap();
    assert_ne!(emulator.ip() as usize, 0);
}

fn test_rc4(bytecode: &[u8]) {
    let mut emulator = Emulator::with_bytecode(bytecode);
    emulator.execute().unwrap();
    assert_ne!(emulator.ip() as usize, 0);
}

fn bm_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    let mut bytecode_map = HashMap::new();
    for &n in &[10, 100, 1000, 10000, 100000, 1000000] {
        bytecode_map.insert(n, compile_fibonacci_bytecode(n));
    }

    for &n in &[10, 100, 1000, 10000, 100000, 1000000] {
        group.bench_function(format!("fibonacci_{}", n), |b| {
            b.iter(|| test_fibonacci(black_box(bytecode_map.get(&n).unwrap())))
        });
    }

    group.finish();
}

fn bm_rust_fibonacci(c: &mut Criterion) {
    fn rust_fibonacci(n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;

        for _ in 0..n {
            let temp = a;
            a = b;
            b = temp + b;
        }

        a
    }

    let mut group = c.benchmark_group("rust_fibonacci");

    for &n in &[10, 100, 1000, 10000, 100000, 1000000] {
        group.bench_function(format!("rust_fibonacci_{}", n), |b| {
            b.iter(|| rust_fibonacci(black_box(n)))
        });
    }

    group.finish();
}

fn bm_rc4(c: &mut Criterion) {
    let mut group = c.benchmark_group("rc4");

    let mut bytecode_map = HashMap::new();
    for &n in &[256, 512, 1024, 2048, 4096] {
        bytecode_map.insert(n, compile_rc4_bytecode(&[0u8; 40], n));
    }

    for &n in &[256, 512, 1024, 2048, 4096] {
        group.bench_function(format!("rc4_{}", n), |b| {
            b.iter(|| test_rc4(black_box(bytecode_map.get(&n).unwrap())))
        });
    }

    group.finish();
}

fn bm_rust_rc4(c: &mut Criterion) {
    fn rc4_init(key: &[u8], sbox: &mut [u8; 256]) {
        for i in 0..256 {
            sbox[i] = i as u8;
        }

        let mut j = 0;
        let key_len = key.len();

        for i in 0..256 {
            j = (j + sbox[i] as usize + key[i % key_len] as usize) % 256;
            sbox.swap(i, j);
        }
    }

    fn rc4_apply_stream(buf: &mut [u8], sbox: &mut [u8; 256]) {
        let mut i = 0;
        let mut j = 0;
        let s = sbox;

        for byte in buf.iter_mut() {
            i = (i + 1) % 256;
            j = (j + s[i] as usize) % 256;

            s.swap(i, j);
            let k = s[(s[i] as usize + s[j] as usize) % 256];
            *byte ^= k;
        }
    }

    fn rust_rc4(buf_len: usize) {
        let key = [0u8; 40];
        let mut sbox = [0u8; 256];
        rc4_init(&key, &mut sbox);
        let mut buf = vec![0u8; buf_len];
        rc4_apply_stream(&mut buf, &mut sbox);
    }

    let mut group = c.benchmark_group("rust_rc4");

    for &n in &[256, 512, 1024, 2048, 4096] {
        group.bench_function(format!("rust_rc4_{}", n), |b| {
            b.iter(|| rust_rc4(black_box(n)))
        });
    }

    group.finish();
}

criterion_group!(fibonacci, bm_fibonacci, bm_rust_fibonacci);
criterion_group!(rc4, bm_rc4, bm_rust_rc4);
criterion_main!(fibonacci, rc4);
