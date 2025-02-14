use std::collections::HashMap;

fn hashmap() {
    // o HashMap é uma espécie de dicionário trabalhando com chave e valor
    let mut mapa: HashMap<String, &str> = HashMap::new();

    // mapa.insert(String::from("nome"), "Joao"); // insert(key, value)
    mapa.insert("nome".to_string(), "Google"); // insert(key, value)
    mapa.insert("url".to_string(), "https://www.google.com");

    // let mut numero: HashMap<u32, &u32> = HashMap::new();

    // numero.insert(0, 1); // insert(key, value)
    // numero.insert(1, 123);
    // numero.insert(2, 2);

    println!("{:?}", mapa);

    // mapa.get(&"url".to_string()) // pega a referência de uma chave retornando um Option

    // match mapa.get(&"url".to_string()) {
    match mapa.get(&"url2".to_string()) {
        Some(valor) => {
            println!("{}", valor);
        }
        None => {
            println!("Não foi possível encontrar o valor!");
        }
    }
}

fn main() {
    hashmap();
}
