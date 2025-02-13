enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32),
}

fn main() {
    enumeracao_com_dados()
}

fn enumeracao_com_dados() {
    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(3, 8, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3d"),
    }

    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2d"),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z),
    }
}
