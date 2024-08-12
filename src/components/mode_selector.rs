use yew::prelude::*;
use gloo_console::log;
use yew::{function_component, html, Html};
use crate::app_state::{AppState, ModeState};

#[function_component(ModeSelector)]
pub fn mode_selector() -> Html {
    let state = use_state(|| AppState::default());

    let change_theme = {
        let state = state.clone();
        //log!("mode selected : ", &state.mode.next_state().to_string());

        Callback::from(move |_| {
            let mode = &state.mode;
            log!("mode selected : ", mode.next_state().to_string()); 
            state.set(AppState {
                mode: mode.next_state(),
            }
        )})
    };

    html! {
        <button onclick={change_theme}>{"Mode"}</button>
    }
}

