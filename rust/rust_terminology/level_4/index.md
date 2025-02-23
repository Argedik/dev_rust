# Seviye 4 - Veri Yapıları (Structs & Enums) (Orta Seviye)

struct	Veri yapıları tanımlamak için kullanılır.
impl	Struct veya enum için metotlar tanımlamak için kullanılır.
enum	Birden fazla olası değeri temsil eden veri yapısı.
derive	Otomatik olarak Debug, Clone gibi trait’leri uygular.
Debug	Debug için println!("{:?}", obj) ile çıktı alınmasını sağlar.
PartialEq	== ve != karşılaştırmalarını mümkün kılar.
Copy	Değerin stack üzerinde kopyalanmasına izin verir.
Clone	clone() çağrılarak nesnenin kopyalanmasını sağlar.
Default	default() metodu ile varsayılan değer döndürmeyi sağlar.
#[derive(Debug)]	Yapıya Debug özelliğini ekler.
#[derive(Clone)]	Yapıya Clone özelliğini ekler.
#[derive(Default)]	Yapıya Default özelliğini ekler.
#[derive(PartialEq)]	Yapıya PartialEq özelliğini ekler.
self	Metot içindeki referansı gösterir.
Self	Yapının kendi tipini ifade eder.
associated functions	Struct veya enum içinde impl ile tanımlanan fonksiyonlar.
methods	self parametresi alan fonksiyonlar.
::new()	Genellikle nesne üretmek için kullanılır.
::default()	Varsayılan nesne üretmek için kullanılır.
#[repr(C)]	Bellek düzenini C dilindeki gibi hizalar.
#[repr(transparent)]	Tek bir alan içeren struct'ları optimize eder.
Tuple Struct	Alanları isimlendirilmemiş struct türü.
Field Struct	Alanları isimlendirilmiş struct türü.
Unit Struct	Veri tutmayan, sadece tür bilgisi içeren struct türü.
PhantomData<T>	Derleyiciye bir tipin varlığını bildirmek için kullanılır