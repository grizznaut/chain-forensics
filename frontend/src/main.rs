use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Deserialize)]
struct Entity {
    name: Option<String>,
}

#[function_component]
fn App() -> Html {
    let entities = use_state(|| vec![]);
    {
        let entities = entities.clone();
        use_effect_with((), move |_| {
            let entities = entities.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_entities: Vec<Entity> = Request::get("/api/entities")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                entities.set(fetched_entities);
            });
            || ()
        });
    }
    html! {
        <>
            <h1>{ "Entities List" }</h1>
            { entities_list((*entities).clone()) }
        </>
    }
}

fn entities_list(entities: Vec<Entity>) -> Html {
    entities
        .into_iter()
        .map(|entity| {
            let name = entity.name.unwrap();
            html! {
                <p key={name.clone()}>{format!("{}", name)}</p>
            }
        })
        .collect()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
