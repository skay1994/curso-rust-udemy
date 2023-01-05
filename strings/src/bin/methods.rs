fn main() {

    // Replace
    let replace = String::from("Olá meu nome é Jorge");
    println!("{}\n", replace);
    println!("{}\n", replace.replace("Jorge", "Carlos"));

    println!();
    // Line breaks
    let linebreaks = "Fui hoje\n ao supermercado\n comprar arroz.";

    for i in linebreaks.lines() {
        println!("A linha é: \"{}\"", i.trim())
    }

    println!();
    // Split
    let split = String::from("Minha+sogra+é+muito+feliz");
    let token: Vec<&str> = split.split("+").collect();

    println!("{:?}", token);
    println!("{}", token[1]);

    println!();
    // Trim
    let trim = "             Meu nome é Jorge         ";
    println!("{}", trim);
    println!("{}", trim.trim());

    println!();

    // Get character by index
    let index = String::from("Deixa uma avaliação de 5 extrelas");
    match index.chars().nth(6) {
        Some(char) => println!("O caracter selecionado é {}", char),
        None => println!("Erro, o caracter não foi localizado."),
    }
}