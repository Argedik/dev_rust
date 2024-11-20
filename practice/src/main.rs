use crate::deneme::test::*;
mod deneme;

fn main() {
  let test = Rvalue::rvalue("deneme");
  println!("{:?}", test.data.to_string())
}