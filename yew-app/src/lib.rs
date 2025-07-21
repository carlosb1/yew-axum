mod websocket;
use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};
use futures::channel::mpsc::Sender;

use std::rc::Rc;
use yew::prelude::*;
use crate::websocket::WebsocketService;

#[derive(Debug, Clone, PartialEq)]
pub struct AppStateInner {
    pub message: String,
    pub tx: Option<Sender<String>>
}

type AppState = Rc<AppStateInner>;


#[function_component]
fn HelloWorld() -> Html {
   let context = use_context::<AppState>().expect("No context found.");
    
    html! { "Hello world" }
}

#[function_component]
pub fn App() -> Html {
    /*  Initialize context */
    let ctx = use_state(|| {
        Rc::new(AppStateInner {
            message: String::from("Welcome to WebAssembly!"),
        })
    });

    {
        let ctx_clone = ctx.clone();
        use_effect(move || {
            let service = WebsocketService::new();
            
        });
    }

    html! {
        <ContextProvider<AppState> context={(*ctx).clone()}>
            <HelloWorld />
        </ContextProvider<AppState>>
    }
}

// Then somewhere else you can use the component inside `html!`


#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
