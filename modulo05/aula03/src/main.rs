struct Pessoa {
    nome: String,
    idade: i8,
    altura: f32,
}

fn main() {
    estrutura();
}

fn estrutura() {
    let glaucio = Pessoa {
        nome: String::from("Glaucio Silva"),
        idade: 25,
        altura: 1.69,
    };

    println!("Nome: {}", glaucio.nome);
    println!("Idade: {}", glaucio.idade);
    println!("Altura: {}", glaucio.altura);
}
