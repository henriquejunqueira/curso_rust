// fn type_of<T>(_variavel: T) -> &'static str {
fn type_of<T>(_: T) -> &'static str {
    // return std::any::type_name::<T>();
    std::any::type_name::<T>()
}

fn main() {
    let inteiro = 10;

    let int_to_float = inteiro as f32; // converte a variável inteiro para f32 (float 32 bits)

    let float = 2.5;

    // ! nesse caso o valor será truncado perdendo o ponto flutuante
    let float_to_int = float as i32; // converte a variável floar para i32 (inteiro 32 bits)

    println!(
        "Valor da variável inteiro: {}, {}",
        inteiro,
        type_of(inteiro)
    );
    println!(
        "Valor da variável int_to_float: {}, {}",
        int_to_float,
        type_of(int_to_float)
    );

    println!("Valor da variável float: {}, {}", float, type_of(float));
    println!(
        "Valor da variável float_to_int: {}, {}",
        float_to_int,
        type_of(float_to_int)
    );

    let int_to_string = inteiro.to_string(); // converte para string

    println!(
        "Valor da variável int_to_string: {}, {}",
        int_to_string,
        type_of(&int_to_string)
    );

    let string = "42";

    let string_to_int = string.parse::<i64>().unwrap();

    println!(
        "Valor da variável string_to_int: {}, {}",
        string_to_int,
        type_of(&string_to_int)
    );
}
