use macros::{Getters, Setters};

#[derive(Getters, Setters)]
struct Test {
    nome: String,
    idade: i32,
}

impl Test {
    pub fn new(nome: String, idade: i32) -> Self {
        Self { nome, idade }
    }
}

fn main() {
    let mut t = Test::new("nome".into(), 18);
    println!("{}, {}", t.nome(), t.idade());
    assert_eq!(*t.nome(), "nome".to_owned());
    assert_eq!(*t.idade(), 18);

    t.set_nome("Avelino de Almeida Bego".into());
    t.set_idade(52);
    assert_eq!(*t.nome(), "Avelino de Almeida Bego".to_owned());
    assert_eq!(*t.idade(), 52);
    println!("{}, {}", t.nome(), t.idade());
}
