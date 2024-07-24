use macros::XML;

#[derive(XML)]
struct Test {
    pub nome: String,
    pub idade: i32,
    pub emprego: Test2,
}

#[derive(XML)]
struct Test2 {
    cargo: String,
    salario: f32,
}

fn main() {
    let t = Test {
        nome: "Avelino".into(),
        idade: 52,
        emprego: Test2 {
            cargo: "programador".into(),
            salario: 10.00,
        },
    };
    println!("{}", t);
}
