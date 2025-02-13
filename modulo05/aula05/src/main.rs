struct Retangulo {
    altura: u32,
    largura: u32,
}

trait FormaGeometrica {
    fn calcular_area(&self) -> u32;

    fn new(largura: u32, altura: u32) -> Self;
}

impl FormaGeometrica for Retangulo {
    fn calcular_area(&self) -> u32 {
        self.largura * self.altura
    }

    fn new(largura: u32, altura: u32) -> Self {
        Self { altura, largura }
    }
}

fn main() {
    estrutura();
}

fn estrutura() {
    // Instância 1
    let retangulo1 = Retangulo {
        largura: 10,
        altura: 20,
    };

    let area1 = retangulo1.calcular_area();

    println!("Rect 1: {}", area1);

    // Fim instância 1

    // Instância 2
    let retangulo2 = Retangulo {
        largura: 33,
        altura: 5,
    };

    let area2 = retangulo2.calcular_area();

    println!("Rect 2: {}", area2);

    // Fim instância 2

    // Instância 3
    let retangulo3 = Retangulo::new(15, 25);

    let area3 = retangulo3.calcular_area();

    println!("Rect 3: {}", area3);

    // Fim instância 3
}
