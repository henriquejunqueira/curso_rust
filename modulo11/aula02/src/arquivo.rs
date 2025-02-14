// use std::{env, fs::File, io::prelude::*}; // io::prelude::* importa tudo da biblioteca std::io
use std::{
    env,
    fs::{File, metadata},
    io::{Read, Write},
};

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

// função para ler arquivo
pub fn ler(caminho_completo: &str) {
    if existe(&caminho_completo).is_ok() {
        match File::open(&caminho_completo) {
            Ok(mut arquivo) => {
                let mut conteudo = String::new();

                arquivo.read_to_string(&mut conteudo).unwrap();

                println!("Arquivo aberto: {}", conteudo);
            }
            Err(e) => {
                println!("Erro ao ler o arquivo: {}", e);
            }
        }
    }
}

// verifica a existência do arquivo
pub fn existe(caminho_completo: &str) -> Result<(), &'static str> {
    if metadata(caminho_completo).is_ok() {
        Ok(())
    } else {
        Err("O arquivo não existe.")
    }
}
