use std::io;
use rand::Rng;

fn main(){
  println!("kullanıcıdan gelen veri");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("hata oluştu.");
  println!("{}", guess);
}