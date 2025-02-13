fn main() {
    // dividir(100, 0);

    // let a: f64 = 0.0;
    let a: f64 = 2.0;

    let resultado_divisao = dividir(100 as f64, a);

    match resultado_divisao {
        Some(valor) => {
            println!("O resultado da divisão é: {}", valor);
        }
        None => {
            println!("Não foi possível fazer a divisão");
        }
    }
}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor) // "Some" signififica se tiver algo, algum valor
    }
}
