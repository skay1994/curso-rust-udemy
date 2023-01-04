struct User (String, String, bool, String);

fn main() {
    let pessoa = User(
        String::from("JoaoPessoa"),
        String::from("joaopessoa@gmail.com"),
        true,
        String::from("Homem")
    );

    println!(
            "O nome do usuario eh {}, seu email eh {}. A conta está ativa? {}",
        pessoa.0,
        pessoa.1,
        pessoa.2
    );
}
