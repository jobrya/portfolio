use yew::{function_component, html, Html};

#[function_component(Bio)]
pub fn bio() -> Html {
    html! { 
        <div class = "name">
            <h2> {"Josh O'Bryant"} </h2>
        </div> 
    }
}