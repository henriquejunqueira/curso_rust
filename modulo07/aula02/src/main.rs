use std::fs::File;
use std::io::{self, Read};

fn main() {
    let resultado = ler_arquivo(
        r"/home/henrique/Documentos/projetos/desenvolvimento/curso_rust/modulo07/aula02/src/main.rs",
    );

    match resultado {
        Ok(conteudo) => {
            println!("Conteúdo do arquivo: {}", conteudo);
        }
        Err(erro) => {
            println!("Erro ao ler arquivo: {}", erro);
        }
    }
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?;

    let mut conteudo = String::new();

    arquivo.read_to_string(&mut conteudo);

    Ok(conteudo)
}
