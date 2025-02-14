fn string() {
    // let texto = String::new(); // cria uma string vazia
    let mut texto = String::from("Olá"); // cria passando um valor inicial pra string

    texto.push_str(" Mundo!!!"); // adiciona mais strings à coleção de str

    println!("{}", texto);
}

fn main() {
    string();
}
