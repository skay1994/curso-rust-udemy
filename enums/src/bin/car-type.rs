enum CarType {
    Fiat,
    Ford,
    Renaut
}

fn nacionalidade_carro(car:CarType) {
    match car {
        CarType::Fiat => println!("O carro eh italiano!"),
        CarType::Ford => println!("O carro eh americano!"),
        CarType::Renaut => println!("O carro eh frances!")
    }
}

fn main() {
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renaut);
}