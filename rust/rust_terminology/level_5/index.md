# Seviye 5 - Sahiplik, Borç Alma ve Yaşam Süresi (İleri Orta Seviye)

ownership	Rust’ın bellek yönetim modelidir.
borrow	Bir değişkenin referansını almak (ödünç almak).
&T	Değiştirilemez referans (immutable borrow).
&mut T	Değiştirilebilir referans (mutable borrow).
move	Değerin sahipliğini bir değişkenden diğerine taşır.
copy	Sahipliği değiştirmeden değeri kopyalar.
clone	Heap üzerindeki veriyi kopyalar.
Rc<T>	Referans sayımı (Reference Counted) için kullanılır.
Arc<T>	Çoklu iş parçacıkları için atomik referans sayımı.
Box<T>	Heap üzerinde veri saklamak için kullanılır.
Deref	* operatörünü overloada izin verir.
Drop	Nesne yok edilirken çağrılan özel metot.
impl Drop	drop metodu tanımlamak için kullanılır.
Cow<T>	Hem sahiplenme (owned) hem referans (borrowed) destekler.
'static	Programın tüm yaşam süresini kapsayan referans.
&'a T	Yaşam süresi (lifetime) anotasyonu.
&'a mut T	Yaşam süresi belirlenmiş mutable referans.
lifetime annotations	Referansların yaşam sürelerini belirtmek için kullanılır.
elision	Derleyicinin otomatik olarak yaşam süresi belirlemesine verilen ad.