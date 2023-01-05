fn main() {
    let numero = 2;

    match numero {
        1 => println!("O numero é 1"),
        2 | 3 => println!("O numero é 2 ou 3"),
        _ => println!("Eu nao sei que numero é"),
    }
}
