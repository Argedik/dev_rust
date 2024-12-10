use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div style="font-family: sans-serif; text-align: center; margin-top: 50px;">
            <h1>{"Hello Yew!"}</h1>
            <p>{ format!("Buton {} kez tıklandı!", *counter) }</p>
            <button {onclick}>{"Tıkla"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
