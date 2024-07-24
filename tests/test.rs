#[test]
fn test() {
    use trybuild::TestCases;
    print!("Testes...");
    let t = TestCases::new();
    t.pass("tests/01-getters.rs");
    t.pass("tests/02-setters.rs");
    t.pass("tests/03-xml.rs");
}
