
mod message; // message.rs dosyasını modül olarak ekliyoruz.
use message::Mesaj; // message modülünden Mesaj veri yapısını kullanıyoruz.

mod input;
use input::Veri;

fn main (){
    let mut mesaj = Mesaj::yeni_mesaj("çoktan başladık");
    println!("{:?}", mesaj);
    println!("{}", mesaj.text);

    mesaj.ekle(" test")
    .buyuk_harf().yazdir().ekle("asd").yazdir();
    mesaj.yazdir();

    Veri::user_data().yazdir();
}
