fn vetor() {
    // cria uma lista. Em [u8; 5] é definido [tipo; tamanho]
    let lista: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Valor na posição 2: {}", lista[2]);

    let mut numeros: Vec<u8> = Vec::new();

    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);

    println!("Valores do vetor: {:?}", numeros);

    for n in numeros {
        println!("{}", n);
    }
}

fn main() {
    vetor();
}
