use yew::{function_component, html, Html, Properties};
use crate::app_state::ModeState;

#[function_component(BioGreet)]
pub fn bio_greet() -> Html {
    html! {
        <div class = "bio-greet"> 
            <div class = "name">
                <h1> {"Josh O'Bryant"} </h1>
            </div>
            <p>{"Hey! 👋 I'm Josh. I make games and web apps."}</p>
        </div> 
    }
}

#[derive(Properties, PartialEq)]
pub struct BioInfoProps {
    pub mode: ModeState,
}

#[function_component(BioInfo)]
pub fn bio_info(props: &BioInfoProps) -> Html {
    html! {
        <div class = "bio-info"> 
            <p>
                {
                    "I’m a software developer in the U.S.A. with a love for solving puzzles. \
                    I hold a degree in Civil Enginnering from Auburn University and have spent the past 3 years honing my skills by maintaining web applications. \
                    I have experience working with full-stack web applications, but I mostly enjoy working with "
                }
                <a href = "https://www.rust-lang.org/" class = {props.mode.link_class()} target = "_blank" rel = "noopener">{"Rust 🦀."}</a>
                {
                    " When I’m not workikng, you can find me on a bike ride. 🚴"
                }
            </p>
            <p>
                {
                    "Currently, I’m focused on creating small games with "
                } 
                <a href = "https://bevyengine.org/" class = {props.mode.link_class()} target = "_blank" rel = "noopener">{"Bevy."}</a>
                {
                    " If you’re interested in collaborating or just want to chat, feel free to email me at "
                }
                <a href = "mailto:jobryant.dev@gmail.com" class = {props.mode.link_class()} target = "_blank" rel = "noopener">{"jobryant.dev@gmail.com."}</a>
            </p>
        </div>

    }
}
