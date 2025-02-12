fn main() {
    declaracao_if();
}

fn declaracao_if() {
    let condicao = true;

    let resultado = if condicao {
        "A condição é verdadeira"
    } else {
        "A condição é falsa"
    }; // se atribuir o if a uma variável é obrigatório o uso de ; (ponto e vírgula)

    println!("{}", resultado);
}
