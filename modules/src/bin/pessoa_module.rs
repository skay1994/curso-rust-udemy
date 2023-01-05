mod pessoa_struct;

use pessoa_struct::Pessoa;

fn main() {
    let pessoa = Pessoa {
        nome: String::from("Jorge"),
        sobrenome: String::from("Carlos"),
        idade: 28,
    };

    pessoa.qual_nome();
    pessoa.nome_completo()
}