#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
