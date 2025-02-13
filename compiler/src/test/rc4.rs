use vm::emulator::Emulator;

use crate::builder::{build_bytecode_s, Builder};

include!("rc4_test_vectors.rs");

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

fn generate_key_source(key: &[u8]) -> String {
    let mut s = String::from("key:\n");

    for c in key {
        s += &format!("db {c}\n");
    }

    s
}

fn generate_buf_source(buf: &[u8]) -> String {
    let mut s = String::from("buf:\n");

    for c in buf {
        s += &format!("db {c}\n");
    }

    s
}

fn test_rc4(key: &[u8], expect: &[&[u8; 32]; 9]) {
    const BUF_SIZE: usize = 4112;
    let buf = [0u8; BUF_SIZE];

    let key_src = generate_key_source(key);
    let buf_src = generate_buf_source(&buf);
    let s = format!(
        "{}\n{key_src}\n{buf_src}\n",
        include_str!("rc4.S")
            .replace("%KEYLEN%", &format!("{}", key.len()))
            .replace("%BUFLEN%", &format!("{}", buf.len()))
    );

    let mut builder = Builder::new();
    build_bytecode_s(s, &mut builder).unwrap();
    builder.finalize().unwrap();
    let dump = builder.dump().unwrap();
    let bytecode_len = dump.len();
    assert_ne!(bytecode_len, 0);

    let key_loc = *builder.labels.get("key").unwrap();
    assert_ne!(key_loc, 0);
    let sbox_loc = *builder.labels.get("sbox").unwrap();
    assert_ne!(sbox_loc, 0);
    let buf_loc = *builder.labels.get("buf").unwrap();
    assert_ne!(buf_loc, 0);

    let mut emulator = Emulator::with_bytecode(dump);
    emulator.execute().unwrap();
    assert_ne!(emulator.ip() as usize, 0);

    let buf = &emulator.dram.0[buf_loc as usize..buf_loc as usize + buf.len()];
    assert_eq!(&buf[0..32], expect[0]);
    assert_eq!(&buf[240..240 + 32], expect[1]);
    assert_eq!(&buf[496..496 + 32], expect[2]);
    assert_eq!(&buf[752..752 + 32], expect[3]);
    assert_eq!(&buf[1008..1008 + 32], expect[4]);
    assert_eq!(&buf[1520..1520 + 32], expect[5]);
    assert_eq!(&buf[2032..2032 + 32], expect[6]);
    assert_eq!(&buf[3056..3056 + 32], expect[7]);
    assert_eq!(&buf[4080..4080 + 32], expect[8]);
}

#[test]
fn rfc6229_1() {
    test_rc4(RFC6229_1_KEY, RFC6229_1_EXPECT);
}

#[test]
fn rfc6229_2() {
    test_rc4(RFC6229_2_KEY, RFC6229_2_EXPECT);
}

#[test]
fn rfc6229_3() {
    test_rc4(RFC6229_3_KEY, RFC6229_3_EXPECT);
}

#[test]
fn rfc6229_4() {
    test_rc4(RFC6229_4_KEY, RFC6229_4_EXPECT);
}

#[test]
fn rfc6229_5() {
    test_rc4(RFC6229_5_KEY, RFC6229_5_EXPECT);
}

#[test]
fn rfc6229_6() {
    test_rc4(RFC6229_6_KEY, RFC6229_6_EXPECT);
}

#[test]
fn rfc6229_7() {
    test_rc4(RFC6229_7_KEY, RFC6229_7_EXPECT);
}

#[test]
fn rfc6229_8() {
    test_rc4(RFC6229_8_KEY, RFC6229_8_EXPECT);
}

#[test]
fn rfc6229_9() {
    test_rc4(RFC6229_9_KEY, RFC6229_9_EXPECT);
}

#[test]
fn rfc6229_10() {
    test_rc4(RFC6229_10_KEY, RFC6229_10_EXPECT);
}

#[test]
fn rfc6229_11() {
    test_rc4(RFC6229_11_KEY, RFC6229_11_EXPECT);
}

#[test]
fn rfc6229_12() {
    test_rc4(RFC6229_12_KEY, RFC6229_12_EXPECT);
}

#[test]
fn rfc6229_13() {
    test_rc4(RFC6229_13_KEY, RFC6229_13_EXPECT);
}

#[test]
fn rfc6229_14() {
    test_rc4(RFC6229_14_KEY, RFC6229_14_EXPECT);
}

mod rust_rc4 {
    use super::*;

    fn test(key: &[u8], expect: &[&[u8; 32]; 9]) {
        let mut sbox = [0u8; 256];
        rc4_init(key, &mut sbox);
        let mut buf = [0u8; 4112];
        rc4_apply_stream(&mut buf, &mut sbox);
        assert_eq!(&buf[0..32], expect[0]);
        assert_eq!(&buf[240..240 + 32], expect[1]);
        assert_eq!(&buf[496..496 + 32], expect[2]);
        assert_eq!(&buf[752..752 + 32], expect[3]);
        assert_eq!(&buf[1008..1008 + 32], expect[4]);
        assert_eq!(&buf[1520..1520 + 32], expect[5]);
        assert_eq!(&buf[2032..2032 + 32], expect[6]);
        assert_eq!(&buf[3056..3056 + 32], expect[7]);
        assert_eq!(&buf[4080..4080 + 32], expect[8]);
    }

    #[test]
    fn rfc6229_1() {
        test(RFC6229_1_KEY, RFC6229_1_EXPECT);
    }

    #[test]
    fn rfc6229_2() {
        test(RFC6229_2_KEY, RFC6229_2_EXPECT);
    }

    #[test]
    fn rfc6229_3() {
        test(RFC6229_3_KEY, RFC6229_3_EXPECT);
    }

    #[test]
    fn rfc6229_4() {
        test(RFC6229_4_KEY, RFC6229_4_EXPECT);
    }

    #[test]
    fn rfc6229_5() {
        test(RFC6229_5_KEY, RFC6229_5_EXPECT);
    }

    #[test]
    fn rfc6229_6() {
        test(RFC6229_6_KEY, RFC6229_6_EXPECT);
    }

    #[test]
    fn rfc6229_7() {
        test(RFC6229_7_KEY, RFC6229_7_EXPECT);
    }

    #[test]
    fn rfc6229_8() {
        test(RFC6229_8_KEY, RFC6229_8_EXPECT);
    }

    #[test]
    fn rfc6229_9() {
        test(RFC6229_9_KEY, RFC6229_9_EXPECT);
    }

    #[test]
    fn rfc6229_10() {
        test(RFC6229_10_KEY, RFC6229_10_EXPECT);
    }

    #[test]
    fn rfc6229_11() {
        test(RFC6229_11_KEY, RFC6229_11_EXPECT);
    }

    #[test]
    fn rfc6229_12() {
        test(RFC6229_12_KEY, RFC6229_12_EXPECT);
    }

    #[test]
    fn rfc6229_13() {
        test(RFC6229_13_KEY, RFC6229_13_EXPECT);
    }

    #[test]
    fn rfc6229_14() {
        test(RFC6229_14_KEY, RFC6229_14_EXPECT);
    }
}
