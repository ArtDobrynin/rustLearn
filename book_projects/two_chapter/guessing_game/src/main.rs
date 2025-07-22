use std::io;

fn main() {
    println!("Guess number!");

    println!("Please, get number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
               .expect("Could not read the line!");

    println!("You guessed: {}", guess)
}
