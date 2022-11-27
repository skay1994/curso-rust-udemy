fn main() {
    let a:i32 = 10;

    {
        println!("O valor de a eh {}", a);
        let a:f32 = 20.309;
        println!("O valor de a eh {}", a);
    }

    println!("O valor de a eh {}", a);
}
