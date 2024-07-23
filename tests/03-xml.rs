use macros::XML;

#[derive(XML)]
struct Test {
    nome: String,
    idade: i32,
}

impl Test {
    pub fn new(nome: String, idade: i32) -> Self {
        Self { nome, idade }
    }
}

fn main(){
    let t = Test::new("Avelino".into(), 52);
    println!("{}", t);
}