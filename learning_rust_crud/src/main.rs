mod model;
mod create;

use model::User;
use create::create_user;

/// Belirli bir ID'ye sahip kullanıcıyı vektörde arar ve döndürür.
/// 
/// # Parametreler
/// - `users`: Kullanıcı listesinin referansı
/// - `id`: Aranacak kullanıcı ID'si
/// 
/// # Dönen değer
/// - `Option<&User>`: Kullanıcı bulunduysa Some(User), bulunmadıysa None
fn read_user(users: &Vec<User>, id: u32) -> Option<&User> {
    users.iter().find(|user| user.id == id)
}

/// Belirli bir ID'ye sahip kullanıcıyı günceller.
/// 
/// # Parametreler
/// - `users`: Kullanıcı listesinin mutable referansı
/// - `id`: Güncellenecek kullanıcı ID'si
/// - `new_name`: Kullanıcının yeni adı
/// 
/// # Dönen değer
/// - `bool`: Güncelleme başarılıysa true, kullanıcı bulunamadıysa false
fn update_user(users: &mut Vec<User>, id: u32, new_name: &str) -> bool {
    if let Some(user) = users.iter_mut().find(|user| user.id == id) {
        user.name = new_name.to_string();
        true
    } else {
        false
    }
}

/// Belirli bir ID'ye sahip kullanıcıyı vektörden siler.
/// 
/// # Parametreler
/// - `users`: Kullanıcı listesinin mutable referansı
/// - `id`: Silinecek kullanıcı ID'si
/// 
/// # Dönen değer
/// - `bool`: Silme başarılıysa true, kullanıcı bulunamadıysa false
fn delete_user(users: &mut Vec<User>, id: u32) -> bool {
    if let Some(pos) = users.iter().position(|user| user.id == id) {
        users.remove(pos);
        true
    } else {
        false
    }
}

fn main() {
    // Kullanıcıları tutmak için boş bir liste oluşturduk
    let mut users: Vec<User> = Vec::new();

    // 1) CREATE: Yeni kullanıcılar ekleyelim
    create_user(&mut users, 1, "Alice");
    create_user(&mut users, 2, "Bob");
    create_user(&mut users, 3, "Charlie");

    println!("Başlangıçta kullanıcı listesi: {:?}", users);

    // 2) READ: ID'si 2 olan kullanıcıyı bulup yazdıralım
    if let Some(user) = read_user(&users, 2) {
        println!("ID'si 2 olan kullanıcı bulundu: {:?}", user);
    } else {
        println!("ID'si 2 olan kullanıcı bulunamadı!");
    }

    // 3) UPDATE: ID'si 3 olan kullanıcının ismini değiştirelim
    let update_success = update_user(&mut users, 3, "Charlie-Updated");
    println!("Kullanıcı güncellendi mi? {:?}", update_success);
    println!("Güncelleme sonrası kullanıcı listesi: {:?}", users);

    // 4) DELETE: ID'si 1 olan kullanıcıyı silelim
    let delete_success = delete_user(&mut users, 1);
    println!("Kullanıcı silindi mi? {:?}", delete_success);
    println!("Silme sonrası kullanıcı listesi: {:?}", users);
}
