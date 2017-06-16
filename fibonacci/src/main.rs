use std::io;
use std::time::Instant;

fn main() {
    println!("Input the length of fibonacci you want to get? !");
    let mut length = String::new();

    io::stdin().read_line(&mut length).expect("Unexpected Error!");

    let length: i32 = length.trim().parse().unwrap();

    let start_at = Instant::now();
    let result = generate_fib(length);
    let elapsed = start_at.elapsed();

    println!(">>> fibonacci generated in {} million seconds: {:?}", (elapsed.as_secs() * 1_000)
        + (elapsed.subsec_nanos() / 1_000_000) as u64, result);
}

fn generate_fib(length: i32) -> Vec<i32> {
    let mut fibonacci: Vec<i32> = vec![];

    for x in 0..length {
        fibonacci.push(fib(x))
    }

    return fibonacci;
}

fn fib(index: i32) -> i32 {
    if index <= 1 { return index };

    return fib(index -2) + fib(index -1);
}
