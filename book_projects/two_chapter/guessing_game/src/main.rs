use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number!");

    let secret_number = rand::rng().random_range(1..101);
    println!("Secret number: {}", secret_number);

    println!("Please, get number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Could not read the line!");

    let guess: u32 = guess.trim().parse()
        .expect("Set number, please!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Less secret number!"),
        Ordering::Greater => println!("More secret number!"),
        Ordering::Equal => println!("You win!")
    }
}
