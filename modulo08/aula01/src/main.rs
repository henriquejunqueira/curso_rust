// Módulo matemática (um módulo sempre é privado, então com pub as funções dentro do
// módulo viram públicas)
mod matematica {

    pub fn somar(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtrair(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    println!("2 + 2 = {}", matematica::somar(2, 2));
    println!("2 - 2 = {}", matematica::subtrair(2, 2));
}
