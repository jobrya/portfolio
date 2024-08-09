use std::ops::Deref;

use yew::prelude::*;

mod app_context;
use app_context::{AppContext, Mode};
mod components;
use components::{
    mode_selector::ModeSelector,
    bio::Bio,
    contacts::Contacts,
    projects::Projects,
};

fn main() {
    yew::Renderer::<App>::new().render();
}


#[function_component]
fn App() -> Html {
    let mode = use_reducer(|| Mode::default());
    let app_context_state = use_state(|| AppContext {
        mode: mode,
    });
    //let app_context = use_context::<AppContext>().expect("failed to get app context");
    //let test2 = app_context.mode.;

    html! {
        <ContextProvider<AppContext> context = {(*app_context_state).clone()}>
            //<main class = {app_context.mode.deref().to_string()}>
            <main>
                <ModeSelector/>
                <Bio/>
                <Contacts/>
                <Projects/>
            </main>
        </ContextProvider<AppContext>>
    }
}