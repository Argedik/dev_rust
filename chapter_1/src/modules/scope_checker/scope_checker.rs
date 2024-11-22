#[allow(dead_code)] // Kullanılmayan fonksiyon uyarısını engelle
#[allow(unused_mut)] // Gereksiz 'mut' uyarısını engelle

pub fn check_scope_behavior () {
  let mut x: u32 = 2; //buradaki mut gereksiz
  println!("Dıştaki x (başlangıç ): {}", x);
  //println!("{{scope içi:}} {}", x);
  {
    let mut x = x;
    x += 1;
    println!("Blok içindeki x: {}", x);
    }
  println!("Blok sonlandıktan sonra dıştaki x: {}", x);
}