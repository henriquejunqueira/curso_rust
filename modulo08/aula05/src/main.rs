// chama o módulo externo matematica.rs dentro da subpasta operações
// use operacoes::matematica::somar;
// use operacoes::matematica::subtrair;

// outra forma de importar o mesmo módulo externo
use operacoes::matematica::{somar, subtrair};
use rand::random; // importa o pacote rand que foi instalado via crates.io

mod operacoes;

fn main() {
    println!("2 + 2 = {}", somar(2, 2));
    println!("2 - 2 = {}", subtrair(2, 2));

    // println!("Número aleatório: {}", random::<i8>());
    println!("Número aleatório: {}", random::<f64>());
}
