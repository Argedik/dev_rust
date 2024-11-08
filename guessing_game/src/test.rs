pub struct Mesaj {
  pub icerik: String,
}

impl Mesaj {
  pub fn yeni(ilk_metin: &str) -> Mesaj {
      Mesaj {
          icerik: ilk_metin.to_string(),
      }
  }

  // Mesaj'a metin ekleyen bir fonksiyon
  pub fn ekle(&mut self, metin: &str) -> &mut Mesaj {
      self.icerik.push_str(metin);
      self // Kendini döndürerek peş peşe çağrılabilir hale getiriyoruz
  }

  // Mesaj'ın içeriğini büyük harfe çeviren bir fonksiyon
  pub fn buyuk_harf(&mut self) -> &mut Mesaj {
      self.icerik = self.icerik.to_uppercase();
      self // Kendini döndürerek peş peşe çağrılabilir hale getiriyoruz
  }

  // İçeriği ekrana yazdıran bir fonksiyon
  pub fn yazdir(&self) {
      println!("{}", self.icerik);
  }
}

pub fn test_main() {
  // `mut` ile değiştirilebilir bir `Mesaj` nesnesi oluşturuyoruz
  let mut mesaj = Mesaj::yeni("Selam");

  // `ekle` ve `buyuk_harf` fonksiyonlarını peş peşe çağırıyoruz
  mesaj
      .ekle(", Rust!")
      .buyuk_harf()
      .ekle(" HARIKA BIR DIL."); // Zincirleme işlemi

  // Sonucu ekrana yazdırıyoruz
  mesaj.yazdir(); // Çıktı: SELAM, RUST! HARIKA BIR DIL.
}