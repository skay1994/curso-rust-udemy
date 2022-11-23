fn dobro(num: i32) -> i32 {
    return 2*num;
}

fn maior(a:i32, b:i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    println!("o dobro de 5 eh {}", dobro(5));
    println!("o maior numero entre 5 e 4 eh {}", maior(5, 4));
}