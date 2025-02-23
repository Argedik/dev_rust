# Koleksiyonlar ve Iteratorler (İleri Orta Seviye)

Vec<T>	Dinamik boyutlu bir vektör veri yapısı.
VecDeque<T>	Çift uçlu kuyruk yapısı.
LinkedList<T>	Bağlı liste veri yapısı.
HashMap<K, V>	Anahtar-değer tabanlı veri saklama.
BTreeMap<K, V>	Sıralı anahtar-değer saklama.
HashSet<T>	Tekil öğelerden oluşan küme veri yapısı.
BTreeSet<T>	Sıralı küme veri yapısı.
BinaryHeap<T>	Öncelikli kuyruk yapısı.
String	Heap üzerinde string veri saklar.
&str	Bellekte statik string saklar.
slice	Veri parçalarını temsil eder (&[T]).
iter()	Değiştirilemez referans iteratörü oluşturur.
iter_mut()	Değiştirilebilir referans iteratörü oluşturur.
into_iter()	Sahiplik alarak iterasyon yapar.
next()	Iterator’un bir sonraki öğesini döndürür.
map()	Iterator’un her öğesini değiştirir.
filter()	Belirli bir koşulu sağlayan öğeleri döndürür.
collect()	Iterator’dan koleksiyon oluşturur.
fold()	Değerleri toplayarak tek bir değer üretir.
enumerate()	Index’leri ve öğeleri birlikte döndürür.
zip()	İki iteratörü birleştirir.