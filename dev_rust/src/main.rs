#[derive(Debug)]
struct Mesaj {
    text: String
}

impl Mesaj {
    fn yeni_mesaj(ilk_mesaj: String)->Mesaj{
        Mesaj{
            text: ilk_mesaj
        }
    }
}

fn main (){
    let mesaj = Mesaj::yeni_mesaj(String::from("çoktan başladık"));
    println!("{:?}", mesaj);
    println!("{}", mesaj.text)

}