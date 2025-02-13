fn main() {
    // funcao_com_panic(10);
    // funcao_com_panic(0); // executa o panic!

    let resultado = std::panic::catch_unwind(|| {
        // let a = funcao_com_panic(0);
        let a = funcao_com_panic(10);

        Ok::<i32, &str>(a)
    });

    match resultado {
        Ok(valor) => {
            println!("Resultado usando result: {}", valor.unwrap());
        }
        Err(_) => {
            println!("A função resultou em panic");
        }
    }
}

fn funcao_com_panic(valor: i32) -> i32 {
    if valor == 0 {
        panic!("O valor não pode ser zero");
    }

    valor
}
