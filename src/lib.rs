use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
