fn main() {
    nome_da_funcao();

    let a = com_retorn();

    println!("O valor retornado em a é: {}", a);

    let b = sem_return();

    println!("O valor retornado em b é: {}", b);
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
