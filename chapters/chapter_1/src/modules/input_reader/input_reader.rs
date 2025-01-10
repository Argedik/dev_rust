#[allow(dead_code)] // Kullanılmayan fonksiyon uyarısını engelle
pub fn read_and_print_input () {
  use std::io;
  println!("{}", "bir değer gir");
  let mut guess= String::new();
  io::stdin().read_line(&mut guess).expect("error");
  println!("{}", guess);
}