use std::collections::HashMap;

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert("Matematica", 90);
    hashmap.insert("Portugues", 72);
    hashmap.insert("Biologia", 58);
    hashmap.insert("Informatica", 96);

    println!("Quantas materias o aluno cursou? {}", hashmap.len());
}