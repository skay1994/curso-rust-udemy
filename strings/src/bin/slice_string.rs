fn main() {
    let nome = "Jorge";
    let nome_string: String = String::from("Jorge Carlos");
    let nome_only = &nome_string[0..5];

    println!("{}", nome_only);
    println!("{}", nome)
}