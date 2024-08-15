use log::{error, info};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    info!("Some info");
    error!("Error message");

    html! {
      <h1>
        {"Hello world!"}
      </h1>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
