#[derive(Debug)]
struct User {
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

impl User {
    fn nome_do_usuario(&self) {
        println!("O nome do usuario é {}", self.username)
    }

    fn ativo_ou_inativo(&self) {
        let status = match self.ativo {
            false => "Não",
            true => "Sim",
        };
        println!("O usuario esta ativo? {}", status)
    }
}

fn main() {
    let mut pessoa = User {
        username: String::from("JoaoPessoa"),
        email: String::from("joaopessoa@gmail.com"),
        ativo: true,
        genero: String::from("Homem"),
    };

    pessoa.nome_do_usuario();
    pessoa.ativo_ou_inativo();

    pessoa.ativo = false;
    pessoa.ativo_ou_inativo();
}
