use rand::Rng;

fn password_generator(n: u128, low: u32, high: u32) -> String {
    assert!(low < high);
    let mut rng = rand::thread_rng();
    let mut output: String = String::new();
    for _ in 0..n {
        output.push(char::from_digit(rng.gen_range(low..high), 10).unwrap())
    }
    return output;
}

fn main() {
    println!("{}", password_generator(10, 0, 10))
}
