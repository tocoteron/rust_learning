use std::io::{self, Write};

fn fibo(n: i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}

fn main() {
    let mut user_input = String::new();

    println!("Calculate fibo(n)");
    print!("Input n: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let n: i32 = user_input.trim().parse().ok().unwrap();

    println!("fibo({}) = {}", n, fibo(n));
}
