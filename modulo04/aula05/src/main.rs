fn main() {
    for_interator();
}

fn for_interator() {
    // loop de 0 a 10 excluindo o 10
    // for item in 0..10 {
    //     println!("O index é: {}", item);
    // }

    // loop de 0 a 10 incluindo o 10
    for item in 0..=10 {
        println!("O index é: {}", item);
    }
}
