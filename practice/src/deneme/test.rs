use std::io;

#[derive(Debug)]
pub struct Rvalue {
  pub data: String
}

impl Rvalue {
  pub fn rvalue (ilk_mesaj: &str ) -> Rvalue {
    let mut guess = String::new();
    println!("{}", ilk_mesaj);
    io::stdin().read_line(&mut guess).expect("olmadı");
    println!("Girilen değer: {}", guess);
    
    Rvalue{
      data: ilk_mesaj.to_string()
    }
  }
}


