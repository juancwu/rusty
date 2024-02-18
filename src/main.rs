mod guessing_game;

fn main() {
    let games = ["guessing game"];

    println!("Let's play a game! Choose:");
    for (index, game) in games.iter().enumerate() {
        println!("{}. {}", index + 1, game);
    }

    guessing_game::play();
}
