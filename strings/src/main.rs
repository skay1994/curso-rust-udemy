fn main() {
    let minhaString: String = String::from("Ola meu nome é Jorge. ");
    println!("o tamanho dessa string eh {}", minhaString.len());
    println!("A minha string está vazia? {}", minhaString.is_empty());

    for token in minhaString.split_whitespace() {
        println!("{}", token);
    }
}