#![allow(non_snake_case)]
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
    log::info!("Application launched - Dioxus");
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, DevBcn!"
        }
    })
}