
#[derive(Debug)]
enum Gender {
    Male, Female
}

fn main() {
    let player_male:Gender = Gender::Male;
    let player_female:Gender = Gender::Female;

    println!("{:?}", player_male);
    println!("{:?}", player_female);
}
