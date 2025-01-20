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
