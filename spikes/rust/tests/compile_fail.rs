#[test]
fn t1_il_non_fidato_non_entra_nel_canale_istruzioni() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
