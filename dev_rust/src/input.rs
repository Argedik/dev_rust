use std::io;

pub struct Veri{
  text: String
}

impl Veri {
  pub fn user_data() -> Veri {
    let mut guess = String::new();
    println!("Bir metin girin:");
    io::stdin().read_line(&mut guess ).expect("Hata oluştu");

    Veri { text: guess.trim().to_string() }
  }

  pub fn yazdir(&self){
    println!("Yazdığın veri: {}", self.text );
  }
}