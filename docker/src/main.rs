use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header style="background-color: #444; color: #fff; padding: 1em; text-align: center;">
                <h1>{ "Rust Blog" }</h1>
            </header>
            <main style="max-width: 800px; margin: 2em auto; font-family: sans-serif;">
                <article>
                    <h2>{ "Web3, NFT, Blockchain, AR/VR, Mining ve Kendi Token/Coin'inizi Oluşturma Üzerine" }</h2>
                    <p>
                      { "Bu blog sayfası Rust dilinde, Yew framework kullanılarak hazırlandı. \
                        Burada Web3, NFT, Blockchain teknolojileri, AR/VR dünyası, Mining süreçleri \
                        ve kendi Token veya Coin'inizi oluşturma gibi konulara değinebilirsiniz." }
                    </p>
                    <p>
                      { "Projedeki değişiklikleri kaydettiğiniz anda trunk sayesinde tarayıcınız otomatik \
                        olarak güncellenecektir. Böylece anlık olarak geliştirdiğiniz blog'un son halini \
                        takip edebilirsiniz." }
                    </p>
                </article>
            </main>
            <footer style="background-color: #444; color: #fff; padding: 1em; text-align: center;">
                { "© 2024 - Rust Blog | Tüm hakları saklıdır." }
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
