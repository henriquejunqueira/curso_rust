fn main() {
    loop_finito();
}

fn loop_finito() {
    let mut contador = 0;

    while contador < 5 {
        println!("Contador: {}", contador);

        contador += 1;
    }
}
