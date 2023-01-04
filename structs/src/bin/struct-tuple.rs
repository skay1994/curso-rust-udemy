struct User (String, String, bool, String);

fn main() {
    let mut pessoa = User(
        String::from("JoaoPessoa"),
        String::from("joaopessoa@gmail.com"),
        true,
        String::from("Homem")
    );

    pessoa.0 = String::from("JoaoPessoa123");

    println!(
        "O nome do usuario eh {}, seu email eh {}. A conta está ativa? {}",
        pessoa.0,
        pessoa.1,
        pessoa.2
    );
}
