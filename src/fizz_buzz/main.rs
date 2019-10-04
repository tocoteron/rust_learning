use std::io::{self, Write};

fn fizz_buzz(n: i32) {
    for i in 1..(n + 1) {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    let mut user_input = String::new();

    println!("Print FizzBuzz, n is max of number");
    print!("Input n: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let n: i32 = user_input.trim().parse().ok().unwrap();

    fizz_buzz(n);
}
