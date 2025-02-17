use std::io;

fn main() {
    println("Guessing Game");
    println("Enter a number");
    let mut guess = string::new();

    io::stdin()
        .readline(&mut guess)
        .expect("Failed to read input");
}