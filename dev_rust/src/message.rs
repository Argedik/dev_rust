/// Mesaj veri yapısı tanımlanıyor.
#[derive(Debug)]
pub struct Mesaj {
    pub text: String
}
impl Mesaj {
  pub fn yeni_mesaj(ilk_mesaj: &str) -> Mesaj {
      Mesaj{
          text: ilk_mesaj.to_string()
      }
  }

  /// Mesaj'a metin ekleyen bir fonksiyon
  pub fn ekle(&mut self, metin: &str) -> &mut Mesaj {
    self.text.push_str( metin);
    self 
  }

  pub fn buyuk_harf(&mut self)-> &mut Mesaj{
    self.text = self.text.to_uppercase();
    self
  }

  pub fn yazdir(&mut self) -> &mut Mesaj {
    println!("{:?}", &self);
    self
  }
}