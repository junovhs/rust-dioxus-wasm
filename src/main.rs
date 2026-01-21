#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::document::eval;

// Define Routes
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    // Init logger
    tracing_wasm::set_as_global_default();
    
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    // Initialize the JS Bridge on mount
    use_effect(move || {
        let _ = eval("window.AppBridge && window.AppBridge.init();");
    });

    rsx! {
        div { 
            style: "display: grid; place-items: center; height: 100vh; font-family: sans-serif;",
            div {
                h1 { "Rust + Dioxus + WASM 🦀" }
                p { "The golden master template is ready." }
            }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { "404 - Not Found" }
    }
}
