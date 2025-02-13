// chama o módulo externo matematica.rs dentro da subpasta operações
// use operacoes::matematica::somar;
// use operacoes::matematica::subtrair;

// outra forma de importar o mesmo módulo externo
use operacoes::matematica::{somar, subtrair};

mod operacoes;

fn main() {
    println!("2 + 2 = {}", somar(2, 2));
    println!("2 - 2 = {}", subtrair(2, 2));
}
