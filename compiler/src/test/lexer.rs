use crate::{builder::build_bytecode_s, Builder};

#[test]
fn lex_label() {
    const S: &str = "
mov r0, 1
test r0, r0
jz mylabel1
mov r0, 3
mylabel1:
mov r0, 7
";
    let mut builder = Builder::new();
    build_bytecode_s(S, &mut builder).unwrap();
    builder.finalize().unwrap();
}

#[test]
fn label_unresolved() {
    const S: &str = "
mov r0, 1
test r0, r0
jz mylabel1
mov r0, 3
mov r0, 7
";
    let mut builder = Builder::new();
    build_bytecode_s(S, &mut builder).unwrap();
    match builder.finalize() {
        Ok(_) => unreachable!(),
        Err(err) => assert_eq!(err, "Unresolved label: mylabel1"),
    };
}
