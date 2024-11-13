use std::io;
use std::cmp::Ordering;

fn main() {
  let mut guess = String::new();

  io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

  let guess: u32 = guess.trim().parse().expect("Please type a number!");

  println!("You guessed: {guess}");

  match guess.cmp(&6) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
  }
}