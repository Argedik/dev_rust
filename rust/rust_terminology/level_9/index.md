Seviye 9 - Unsafe Rust ve Düşük Seviye Programlama (Usta Seviye)

unsafe	Güvensiz işlemleri yürütmek için kullanılır.
raw pointers	*mut T, *const T ile bellek adreslerine doğrudan erişim sağlar.
extern "C"	C diline bağlanmak için kullanılır.
#[no_mangle]	Fonksiyon adının değiştirilmemesini sağlar.
volatile_read
volatile_write
static mut
AtomicUsize
AtomicBool
LLVM IR
inline assembly
FFI (Foreign Function Interface)
bindgen
std::ffi
std::mem::transmute
std::ptr::null  Boş pointer döndürür.
std::ptr::null_mut
align_of
size_of
slice::from_raw_parts