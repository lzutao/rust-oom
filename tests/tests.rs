// Miri cannot run trybuild
#[cfg_attr(miri, ignore)]
#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
    t.compile_fail("tests/compile-fail/boundless-lifetimes/*.rs");
}
