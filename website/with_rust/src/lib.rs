use yew::prelude::*;

struct App {
    name: String,
}

enum Msg {
    UpdateName(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "Ziyaretçi".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateName(input.value())
        });

        html! {
            <div>
                <h1>{"Merhaba, "}{&self.name}{"!"}</h1>
                <input type="text" oninput={on_input} placeholder="Adınızı girin" />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
