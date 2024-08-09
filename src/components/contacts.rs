use yew::{function_component, html, Html};

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! { 
        <div class = "contacts">
            <h2> {"contacts"} </h2>
        </div> 
    }
}