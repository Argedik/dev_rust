use warp::Filter;

#[tokio::main]
async fn main() {
    // `index.html`'i sun
    let html_route = warp::fs::file("./static/index.html");
    let css_route = warp::fs::file("./static/styles.css");

    // Tüm dosyaları sunma
    let routes = warp::path("static")
        .and(html_route.or(css_route));

    println!("Sunucu başlatıldı: http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}