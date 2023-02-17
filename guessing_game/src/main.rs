use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess");
    println!("Please enter the number.");

    let mut guess = String::new();
    let random_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed");

    println!("You guessed: {guess}");

    match guess.cmp(&random_number) {
      Ordering::Less => println!("Too low"),
      Ordering::Greater => println!("Too high"),
      Ordering::Equal => println!("You win"),
    }
}
