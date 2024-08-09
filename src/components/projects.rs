use yew::{function_component, html, Html};

#[function_component(Projects)]
pub fn projects() -> Html {
    html! { 
        <div class = "projects">
            <h2> {"a project"} </h2>
        </div> 
    }
}