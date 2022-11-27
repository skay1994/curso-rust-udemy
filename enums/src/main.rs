enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player:Direction =Direction::Right;

    match player {
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Right => println!("O jogador foi para direita"),
        Direction::Left => println!("O jogador foi para esquerda"),
    }
}
