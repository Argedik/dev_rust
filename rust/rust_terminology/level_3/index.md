# Fonksiyonlar ve Tuple Kullanımı (Orta Seviye)

fn	Fonksiyon tanımlamak için kullanılır.
main	Programın başlangıç fonksiyonudur.
->	Fonksiyonun dönüş türünü belirtir.
()	Boş bir tuple (unit type).
(T, U, ..)	Tuple veri yapısı.
.0, .1, .2	Tuple içindeki öğelere erişim sağlar.
Result<T, E>	Hata yönetimi için kullanılan tür.
Option<T>	Değer olup olmadığını ifade eden tür.
Some	Option<T> içinde değer olduğunu gösterir.
None	Option<T> içinde değer olmadığını gösterir.
unwrap()	Result veya Option içindeki değeri alır.
expect()	Hata mesajı ile birlikte unwrap işlemi yapar.
map()	Option veya Result içindeki değeri işler.
ok()	Result<T, E>'yi Option<T>'ye çevirir.
err()	Result<T, E>'yi Option<E>'ye çevirir.
?	Hata propagasyonu yapar.