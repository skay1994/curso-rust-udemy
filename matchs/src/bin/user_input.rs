use std::io;

fn main() {
    let mut messagem_usuario = String::new();

    println!("Ei usuario, digite algo:\n");

    match io::stdin().read_line(&mut messagem_usuario) {
        Ok(_) => println!("O conteudo digitado é: {}", messagem_usuario),
        Err(e) => println!("Deu erro. O erro é {}", e)
    }
}