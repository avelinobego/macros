/*
TODO: Criar aqui toda a lógica para gerar um XML a partir de uma struct.
Deve haver condições de gear um name space a partir de um propriedade específica.
(Re)implementar a trait "Display" para gerar o XML.
*/

#[test]
fn test() {
    use trybuild::TestCases;
    print!("Testes...");
    let t = TestCases::new();
    t.pass("tests/01-getters.rs");
    t.pass("tests/02-setters.rs");
    t.pass("tests/03-xml.rs");
}
