use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess");
  let random_number = rand::thread_rng().gen_range(1..=100);

  loop {
    println!("Please guess the number: ");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed");

    println!("You entered: {guess}");

    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&random_number) {
      Ordering::Less => println!("Too low"),
      Ordering::Greater => println!("Too high"),
      Ordering::Equal => {
        println!("You win");
        break;
      }
    }
  }
}
