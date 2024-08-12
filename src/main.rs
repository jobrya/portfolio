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
            <div class = "mode-selector">
                <button class = {state.mode.button_class()} onclick={change_theme}></button>
            </div>
            <div class = "selfie-container">
                <img src = "./assets/josh.jpg" alt = "picture of Josh" class = "selfie"/>
            </div>
            <div class = "main-column">
                <BioGreet/>
                <Contacts mode = {state.mode.clone()}/>
                <BioInfo mode = {state.mode.clone()}/>
                <Projects/>
            </div>
        </main>
    }
}