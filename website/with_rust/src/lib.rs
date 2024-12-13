<<<<<<< HEAD
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    web_sys::window()
        .expect("window not available")
        .document()
        .expect("document not available")
        .get_element_by_id("output")
        .expect("output element not found")
        .set_inner_html("<h1>Hello, Rust + WASM!</h1>");
}
=======
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
>>>>>>> 44960a8e483968830221ea1a2335a19f51e736f3
