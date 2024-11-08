mod test;
use std::io;
use test::Mesaj;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    test::test_main();

  // `Mesaj` yapısını kullanma
    let mut mesaj = Mesaj::yeni("Rust öğrenmek");
    mesaj
        .ekle(" çok eğlenceli")
        .buyuk_harf()
        .ekle(" VE ÖĞRETİCİ.");
  mesaj.yazdir(); // Çıktı: RUST ÖĞRENMEK ÇOK EĞLENCELİ VE ÖĞRETİCİ.
}

