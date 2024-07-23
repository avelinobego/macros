use macros::Getters;


#[derive(Getters)]
struct Test {
    nome: String,
}

impl Test {
    pub fn new(nome: String) -> Self {
        Self { nome }
    }
}

fn main() {
    let t = Test::new("Avelino Bego".into());

    println!("{}", t.nome());
    assert_eq!(*t.nome(), String::from("Avelino Bego"));
}
