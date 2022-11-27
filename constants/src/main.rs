const NUMERO_PI:f32 = 3.14159;

fn comprimento_circuferencia(r:f32) -> f32 {
    let c = 2f32*r*NUMERO_PI;
    c
}

fn main() {
    println!("A circuferencia de raio 2.32 possui comprimento de {}", comprimento_circuferencia(2.32))
}
