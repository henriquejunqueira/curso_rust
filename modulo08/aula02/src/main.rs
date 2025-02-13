mod matematica; // chama o módulo externo matematica.rs

fn main() {
    println!("2 + 2 = {}", matematica::somar(2, 2));
    println!("2 - 2 = {}", matematica::subtrair(2, 2));
}
