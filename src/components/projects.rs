use yew::{function_component, html, Html, Properties};
use crate::app_state::ModeState;

#[derive(Clone, PartialEq)]
struct Project {
    name: String,
    source_link: String,
    demo_link: String,
    info: String,
    credits: Vec<Credit>,
}

#[derive(Clone, PartialEq)]
struct Credit {
    name: String,
    link: String,
}

#[derive(Properties, PartialEq)]
pub struct CreditCompProps {
    credit: Credit,
    mode: ModeState,
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
            source_link: String::from(""),
            demo_link: String::from("joshobryant"),
            info: String::from("This website! A place to showcase my work."),
            credits: vec![
                Credit {
                    name: String::from("Yew"),
                    link: String::from("https://yew.rs/"),
                },
                Credit {
                    name: String::from("Gloo"),
                    link: String::from("https://gloo-rs.web.app/"),
                },
                Credit {
                    name: String::from("Trunk"),
                    link: String::from("https://trunkrs.dev/"),
                },
            ]
        },
         Project {
            name: String::from("uppy-up"),
            source_link: String::from(""),
            demo_link: String::from("uppy"),
            info: String::from("A simle game where the player moves up before time runs out. Built with Bevy."),
            credits: vec![
                Credit {
                    name: String::from("Bevy"),
                    link: String::from("https://bevyengine.org/"),
                },
            ]
        },
         Project {
            name: String::from("task board api"),
            source_link: String::from(""),
            demo_link: String::from(""),
            info: String::from("An example Rust API that uses HTTP requests to alter a Postgres database. Built with Actix and sqlx."),
            credits: vec![
                Credit {
                    name: String::from("Actix"),
                    link: String::from("https://actix.rs/"),
                },
                Credit {
                    name: String::from("Serde"),
                    link: String::from("https://serde.rs/"),
                },
                Credit {
                    name: String::from("sqlx"),
                    link: String::from("https://docs.rs/sqlx/latest/sqlx/"),
                },
                Credit {
                    name: String::from("futures"),
                    link: String::from("https://docs.rs/futures/latest/futures/"),
                },
            ]
        },
    ]};
    
    html! { 
        <div class = "projects">
            <h2> {"Projects"} </h2>
            {
                projects.iter().map(|project| 
                {
                    html! {
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
                <p>
                    {"Built with: "}
                    {
                        props.project.credits.iter().map(|credit| 
                        {
                            html! {
                                <>
                                    <CreditComp credit = {(*credit).clone()} mode = {props.mode.clone()}/>
                                    if credit != props.project.credits.last().unwrap() {
                                        {", "}
                                    } 
                                </>
                            }
                        }).collect::<Html>()
                    }
                </p>
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


#[function_component(CreditComp)]
pub fn credit(props: &CreditCompProps) -> Html {
    html! {
        <a href = {props.credit.link.clone()} class = {props.mode.link_class()} target = "_blank" rel = "noopener">
            {props.credit.name.clone()}
        </a>
    }
}