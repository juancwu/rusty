use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("There is a randomly selected number between 1 to 100");

    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please enter a number.");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
