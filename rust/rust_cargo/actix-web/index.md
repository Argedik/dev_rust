[dependencies]
actix-web = "4"
22.02.2025

https://actix.rs/docs/

Şuanda Http server oluşturmak için kullanıyoruz. 22.02.2025

# Açıklama
 Actix-Web, Rust programlama diliyle yazılmış, hızlı ve asenkron çalışan bir web framework'üdür. Rust dilinin sunduğu yüksek performans ve güvenlik avantajlarını kullanarak modern web uygulamaları geliştirmek için tasarlanmıştır.
# Özellikleri

Asenkron çalışır → tokio tabanlıdır, çoklu bağlantıları verimli yönetir.
Hızlıdır → Benchmarks testlerinde en hızlı Rust web framework'lerinden biridir.
Güvenlidir → Rust’ın bellek güvenliği avantajlarından faydalanır.
Modülerdir → Mikro servisler, REST API'ler ve WebSocket uygulamaları için uygundur.
HTTP/2 ve WebSocket desteği sunar.

# Actix-Web gelişimi:

Sürüm	Tarih	Öne Çıkan Özellikler
0.5	2018	İlk kararlı sürüm.
1.0	2019	Büyük stabilite güncellemesi.
2.0	2020	Performans iyileştirmeleri.
3.0	2021	WebSocket ve HTTP/2 desteği.
4.0	2022 Mart	En güncel büyük sürüm, Rust 2021 desteği.


# Actix-Web Kullanım Alanları
Actix-Web, performans ve güvenlik gerektiren web projeleri için uygundur. İşte bazı örnek kullanım alanları:

1️⃣ RESTful API Geliştirme

Kullanıcı yönetimi API'leri
E-ticaret API'leri
IoT cihazlarından veri toplama
2️⃣ Gerçek Zamanlı WebSocket Uygulamaları

Canlı sohbet sistemleri
Gerçek zamanlı finans verileri
Oyun sunucuları
3️⃣ Mikro Servis Mimarisi

Küçük, bağımsız hizmetler olarak çalışan sistemler
Büyük projelerin ölçeklenebilirliği için ideal
4️⃣ Statik Dosya Sunucusu

Web siteleri ve statik içerik barındırma


web, App, HttpResponse, HttpServer, Responder
