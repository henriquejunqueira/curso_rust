fn main() {
    se();
}

fn se() {
    // let numero = 99;
    let numero = 0;

    if numero > 0 {
        println!("O número {} é positivo.", numero);
    } else if numero == 0 {
        println!("O número é zero.");
    } else {
        println!("O número {} é negativo.", numero);
    }
}
