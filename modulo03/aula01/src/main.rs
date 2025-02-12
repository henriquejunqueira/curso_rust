fn main() {
    nome_da_funcao();

    let a = com_retorn();

    println!("O valor retornado em a é: {}", a);

    let b = sem_return();

    println!("O valor retornado em b é: {}", b);

    let c = maior_valor(10, 20);

    println!("O maior valor entre {} e {} é {}", 10, 20, c);
}

fn nome_da_funcao() {
    println!("Olá Henrique!");
}

fn com_retorn() -> i8 {
    return 3;
}

fn sem_return() -> i8 {
    // 3
    10
}

fn maior_valor(a: i32, b: i32) -> i32 {
    // { if a > b { a } else { b }}
    if a > b { a } else { b }
}
