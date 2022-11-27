fn main() {
    let tupla = (12, "valores", 3.14, (1,2,3));
    let (a, b, c, d) = tupla;
    println!("O valor de a eh {}", a);
    println!("O valor de b eh {}", b);
    println!("O valor de c eh {}", c);
    println!("O valor de d eh {:?}", d);
}