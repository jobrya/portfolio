use yew::prelude::*;
use yew::{function_component, html, Html};
use crate::app_context::{AppContext, ModeAction};

#[function_component(ModeSelector)]
pub fn mode_selector() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");

    let change_theme = {
        let app_context = app_context.clone();

        Callback::from(move |_| {
            app_context.mode.dispatch(ModeAction::Switch)
        })
    };

    html! {
        <button onclick={change_theme}>{"Mode"}</button>
    }
}

