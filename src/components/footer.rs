use yew::{function_component, html, Html, Properties};
use crate::app_state::ModeState;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub mode: ModeState,
}

#[function_component(Footer)]
pub fn footer(_props: &FooterProps) -> Html {
    html! {
        <div class = "footer">
            <p> {"this is a footer"}
            </p>
        </div>
    }
}