use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn play() {
    println!("Welcome to guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please enter your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("Your guess: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }
}
