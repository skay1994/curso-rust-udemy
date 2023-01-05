extern crate rand;

use rand::Rng;

fn main() {
    let valores_randomicos = rand::thread_rng().gen_range(5..11);

    println!("{}", valores_randomicos);

    let booleano_randomico = rand::thread_rng().gen_bool(1.0 / 10.0);
    println!("O resulta do é: {}", booleano_randomico)
}
