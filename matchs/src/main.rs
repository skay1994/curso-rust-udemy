fn main() {
    let numero = 6;

    match numero {
        1 => println!("O numero é 1"),
        2 | 3 => println!("O numero é 2 ou 3"),
        4..=10 => println!("O numero entre 4 ou 10"),
        _ => println!("Eu nao sei que numero é"),
    }
}
