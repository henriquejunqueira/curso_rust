fn main() {
    let d = 5;

    let resultado_incremento = incrementa(d);

    println!(
        "Número original: {}, Número incrementado: {}",
        d, resultado_incremento
    );
}

fn incrementa(mut a: i32) -> i32 {
    a += 1;
    a
}
