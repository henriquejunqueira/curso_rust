fn main() {
    let c = maior_valor(10, 20);

    println!("O maior valor entre {} e {} é {}", 10, 20, c);
}

fn maior_valor(a: i32, b: i32) -> i32 {
    // { if a > b { a } else { b }}
    if a > b {
        a
    } else {
        b
    }
}
