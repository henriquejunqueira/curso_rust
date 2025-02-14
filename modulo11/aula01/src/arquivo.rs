// use std::{env, fs::File, io::prelude::*}; // io::prelude::* importa tudo da biblioteca std::io
use std::{env, fs::File, io::Write};

// pega o caminho do usuário
pub fn obter_caminho_usuario() -> Option<String> {
    if let Some(caminho_home) = env::var_os("HOME") {
        Some(caminho_home.into_string().unwrap())
    } else {
        None
    }
}

// função pública de criação de arquivo
pub fn criar(caminho: &str, nome_arquivo: &str) {
    // println!("Criar arquivo na pasta: {}", caminho);
    println!("Criando arquivo no caminho: {}", caminho);
    println!("Criando arquivo com o nome: {}", nome_arquivo);

    // formatando o caminho do arquivo
    let caminho_completo = format!(r"{}/{}", caminho, nome_arquivo);

    // criando o arquivo
    match File::create(&caminho_completo) {
        Ok(_) => {
            println!("Arquivo criado com sucesso no caminho {}", caminho_completo);
        }
        Err(e) => {
            println!("Erro ao criar o arquivo: {}", e);
        }
    }
}
