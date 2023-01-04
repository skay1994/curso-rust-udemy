use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut arquivo = File::open("text.txt").expect("Não conseguiu ler o arquivo.");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Não conseguiu ler o conteudo do arquivo");

    println!("O conteudo do arquivo é: \n\n\n {}", conteudo)
}
