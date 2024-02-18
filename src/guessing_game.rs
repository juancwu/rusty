use rand::Rng;

pub fn play() {
    println!("Welcome to guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("{}", secret_number);
}
