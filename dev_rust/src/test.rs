/// İlk metni alır ve `Mesaj` struct'ı döndürür.
#[derive(Debug)]
struct Mesaj {
    text: String
}
/// Yeni bir mesaj oluşturur. 
impl Mesaj{
    fn yeni(ilk_metin: String)-> Mesaj{
        Mesaj{
            text: ilk_metin
        }
    }
}
fn main (){
    let mesaj = Mesaj::yeni(String::from("Merhaba, Dünya!"));
    println!("{:?}", mesaj);
    println!("{}", mesaj.text);
}

