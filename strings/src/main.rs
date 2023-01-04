fn main() {
    let mut minhaString: String = String::from("Ola meu nome é Jorge. ");
    println!("o tamanho dessa string eh {}", minhaString.len());
    println!("A minha string está vazia? {}", minhaString.is_empty());

    for token in minhaString.split_whitespace() {
        println!("{}", token);
    }

    println!("O nome 'Jorge' está contido na String? {}", minhaString.contains("Jorge"));

    minhaString.push_str("Bem vindo a aula. ");

    println!("{}", minhaString)
}
