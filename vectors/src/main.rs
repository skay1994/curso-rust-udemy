fn main() {
    let mut vetores = vec![1,2,3,4];
    let mut vetores2: Vec<i32> = Vec::new();

    println!("{}", vetores[0]);

    vetores.push(5);

    println!("{}", vetores[4]);

    vetores.remove(1);

    println!("{:?}", vetores);

    for i in vetores.iter() {
        println!("{}",i);
    }
}
