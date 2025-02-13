fn main() {
    deu_match();
}

fn deu_match() {
    let estacao_atual = "verão";

    // o match é tipo o switch de outras linguagens
    match estacao_atual {
        "primavera" => {
            println!("É primavera!");
        }
        "verão" => {
            println!("É verão!");
        }
        "outono" => {
            println!("É outono!");
        }
        "inverno" => {
            println!("É inverno!");
        }
        _ => {
            // valor padrão
            println!("Estação desconhecida!");
        }
    }
}
