#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum.rs");
    t.pass("tests/struct.rs");
}