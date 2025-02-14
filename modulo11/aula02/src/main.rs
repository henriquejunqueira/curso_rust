mod arquivo;

use arquivo::{criar, existe, ler, ler_diretorio, obter_caminho_usuario};

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    if Ok(()) == existe(&r"/home/henrique/henrique.txt") {
        println!("Arquivo existe!");
    } else {
        println!("Arquivo não existe!");
        criar(&caminho, &"henrique.txt");
    }

    // criar(&"Hello, world!");
    // criar(&caminho, &"henrique.txt");

    ler(&r"/home/henrique/henrique.txt");

    match ler_diretorio(&caminho) {
        Ok(_) => println!("Leitura OK"),
        Err(_) => println!("Falha na leitura"),
    }
}
