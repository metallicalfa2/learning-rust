use std::io;

fn main() {
    println!("Guess the game, Please input a number !!");

    let mut guess = String::new();
 
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you have guessed: {}", guess)
}
