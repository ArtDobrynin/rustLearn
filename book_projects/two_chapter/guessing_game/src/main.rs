use std::io;
use rand::Rng;

fn main() {
    println!("Guess number!");

    let secret_number = rand::rng().random_range(1..101);
    println!("Secret number: {}", secret_number);

    println!("Please, get number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
               .expect("Could not read the line!");

    println!("You guessed: {}", guess)
}
