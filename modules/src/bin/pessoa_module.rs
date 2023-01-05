mod pessoa_struct;

fn main() {
    let pessoa = pessoa_struct::Pessoa {
        nome: String::from("Jorge"),
        sobrenome: String::from("Carlos"),
        idade: 28,
    };

    pessoa.qual_nome();
    pessoa.nome_completo()
}