use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut bananas = String::new();

    io::stdin()
        .read_line(&mut bananas)
        .expect("Failed to read line");

    println!("You guessed: {bananas}");    
}