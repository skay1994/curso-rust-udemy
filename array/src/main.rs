fn main() {
    let numeros_inteiros: [i32; 5] = [2;5];
    println!("{}\n", numeros_inteiros[4]);

    for i in numeros_inteiros.iter() {
        println!("{}", i);
    }
}
