mod module;

use module::structs::Pessoa;

fn main() {
    let pessoa = Pessoa::Pessoa {
        nome: String::from("Jorge"),
        sobrenome: String::from("Carlos"),
        idade: 28,
    };

    pessoa.qual_nome();
    pessoa.nome_completo()
}