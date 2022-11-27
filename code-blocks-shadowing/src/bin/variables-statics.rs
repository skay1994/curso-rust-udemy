static mut STATIC_VARIABLE:i32 = 15;

fn main() {
    unsafe {
        println!("O valor da STATIC_VARIABLE eh {}", STATIC_VARIABLE)
    }
}