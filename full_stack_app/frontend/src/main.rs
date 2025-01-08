use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct User {
    id: i64,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let new_name = use_state(|| "".to_string());

    {
        // Component mount olduğunda kullanıcı listesini çekelim
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("http://127.0.0.1:3000/users")
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if let Ok(api_response) = response.json::<ApiResponse<Vec<User>>>().await {
                                if let Some(user_list) = api_response.data {
                                    users.set(user_list);
                                }
                            }
                        }
                        Err(err) => {
                            gloo_console::log!(format!("Fetch hata: {:?}", err));
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let oninput_name = {
        let new_name = new_name.clone();
        Callback::from(move |e: InputEvent| {
            let input_value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            new_name.set(input_value);
        })
    };

    let onclick_create = {
        let users = users.clone();
        let new_name = new_name.clone();
        Callback::from(move |_| {
            let users = users.clone();
            let new_name = new_name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let body = serde_json::json!({ "name": (*new_name).clone() });
                match Request::post("http://127.0.0.1:3000/users")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(api_response) = resp.json::<ApiResponse<User>>().await {
                            if let Some(created_user) = api_response.data {
                                // Yeni user'ı listeye ekleyelim:
                                let mut current_users = (*users).clone();
                                current_users.push(created_user);
                                users.set(current_users);
                                new_name.set("".to_string());
                            }
                        }
                    }
                    Err(e) => {
                        gloo_console::log!(format!("POST hata: {:?}", e));
                    }
                }
            });
        })
    };

    html! {
        <div style="margin: 20px;">
            <h1>{ "Yew + Axum + SQLx Basit CRUD" }</h1>
            <div>
                <input
                    type="text"
                    placeholder="Kullanıcı adı"
                    value={(*new_name).clone()}
                    oninput={oninput_name}
                />
                <button onclick={onclick_create}>{ "Ekle" }</button>
            </div>

            <h2>{ "Kullanıcı Listesi" }</h2>
            <ul>
            {
                for users.iter().map(|user| {
                    html! {
                        <li key={user.id}>
                            { format!("ID: {}, Name: {}", user.id, user.name) }
                        </li>
                    }
                })
            }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
