use yew::{function_component, html, Html, Properties};
use crate::app_state::ModeState;

#[derive(Clone, PartialEq)]
struct Project {
    name: String,
    info: String,
    source_link: String,
    demo_link: String,
}

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    pub mode: ModeState,
}

#[derive(Properties, PartialEq)]
pub struct ProjectCompProps {
    project: Project,
    mode: ModeState,
}

#[function_component(Projects)]
pub fn projects(props: &ProjectProps) -> Html {

    let projects: Vec<Project> = {vec![
        Project {
            name: String::from("portfolio"),
            info: String::from("This website! A place to showcase my work. Built with Yew."),
            source_link: String::from(""),
            demo_link: String::from("joshobryant"),
        },
         Project {
            name: String::from("uppy-up"),
            info: String::from("A simle game where the player moves up before time runs out. Built with Bevy."),
            source_link: String::from(""),
            demo_link: String::from("uppy"),
        },
         Project {
            name: String::from("task board api"),
            info: String::from("An example Rust API that uses HTTP requests to alter a Postgres database. Built with Actix and sqlx."),
            source_link: String::from(""),
            demo_link: String::from(""),
        },
    ]};
    
    html! { 
        <div class = "projects">
            <h2> {"Projects"} </h2>
            {
                projects.iter().map(|project| 
                {
                    html!{
                        <ProjectComp project = {(*project).clone()} mode = {props.mode.clone()}/>
                    }
                }).collect::<Html>()
            }
        </div> 
    }
}

#[function_component(ProjectComp)]
pub fn project(props: &ProjectCompProps) -> Html {
    html! {
        <div class = {props.mode.accent_class()}>
            <div class = "project">
                <h3>{props.project.name.clone()}</h3>
                <p>{props.project.info.clone()}</p>
                <div class = "project-links">
                    <div class = "project-link">
                        <a href = {props.project.source_link.clone()} target = "_blank" rel = "noopener">
                            <img src = {props.mode.source_code_src()} alt = {"source code icon"} class = {props.mode.icon_class()}/>
                        </a>
                    </div>
                    if !props.project.demo_link.is_empty() {
                        <div class = "project-link">
                            <a href = {props.project.demo_link.clone()} target = "_blank" rel = "noopener">
                                <img src = {props.mode.demo_src()} alt = {"demo icon"} class = {props.mode.icon_class()}/>
                            </a>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}