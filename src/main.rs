use yew::prelude::*;
use gloo_console::log;
mod app_state;
use app_state::AppState;

mod components;
use components::{
    bio::{BioGreet, BioInfo},
    contacts::Contacts,
    projects::Projects,
    footer::Footer,
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
            <div class = "control-container">
                <a href = "#home">
                    <img src = {state.mode.home_src()} alt = {"home icon"} class = {state.mode.icon_class()}/>
                </a>
                <button class = {"hide-button"} onclick={change_theme}>
                    <img src = {state.mode.mode_src()} alt = {"mode icon"} class = {state.mode.icon_class()}/>
                </button>
                <a href = "#projects">
                    <img src = {state.mode.folder_src()} alt = {"projects icon"} class = {state.mode.icon_class()}/>
                </a>      
            </div>
            <div class = "selfie-container">
                <img src = "./assets/josh.jpg" alt = "picture of Josh" class = "selfie"/>
            </div>
            <div id = "home" class = "main-column">
                <BioGreet/>
                <Contacts mode = {state.mode.clone()}/>
                <BioInfo mode = {state.mode.clone()}/>
                <Projects mode = {state.mode.clone()}/>
                <Footer mode = {state.mode.clone()}/>
            </div>
        </main>
    }
}