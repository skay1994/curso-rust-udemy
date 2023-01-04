#[derive(Debug)]
struct User {
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

fn main() {
    let pessoa = User {
        username: String::from("JoaoPessoa"),
        email: String::from("joaopessoa@gmail.com"),
        ativo: true,
        genero: String::from("Homem"),
    };

    println!(
        "O nome do usuario eh {}, seu email eh {} e seu genero eh {}",
        pessoa.username,
        pessoa.email,
        pessoa.genero
    );
}
