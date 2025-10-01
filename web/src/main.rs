use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    App {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        body {
            h1 { "Hoi" }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
