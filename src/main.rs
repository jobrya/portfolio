use yew::prelude::*;
use gloo_console::log;
mod app_state;
use app_state::AppState;

mod components;
use components::{
    bio::{BioGreet, BioInfo},
    contacts::Contacts,
    projects::Projects,
};

fn main() {
    yew::Renderer::<App>::new().render();
}


#[function_component]
fn App() -> Html {
    let state = use_state(|| AppState::default());

    let change_theme = {
        let state = state.clone();

        Callback::from(move |_| {
            let mode = &state.mode;
            log!("mode selected :", mode.next_state().to_string()); 
            state.set(AppState {
                mode: mode.next_state(),
            }
        )})
    };

    html! {
        <main class = {state.mode.to_string()}>
            <div class = "mode-container">
                <button class = {"hide-button"} onclick={change_theme}>
                    <img src = {state.mode.mode_src()} alt = {"mode icon"} class = {state.mode.icon_class()}/>
                </button>
            </div>
            <div class = "selfie-container">
                <img src = "./assets/josh.jpg" alt = "picture of Josh" class = "selfie"/>
            </div>
            <div class = "main-column">
                <BioGreet/>
                <Contacts mode = {state.mode.clone()}/>
                <BioInfo mode = {state.mode.clone()}/>
                <Projects mode = {state.mode.clone()}/>
            </div>
            // <footer>
            //     <p>{"bottom of the page"}
            //     </p>
            // </footer>
        </main>
    }
}