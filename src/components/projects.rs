use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
struct Project {
    name: String,
    info: String,
    link: String,
}

#[derive(Properties, PartialEq)]
pub struct ProjectCompProps {
    project: Project,
}

#[function_component(Projects)]
pub fn projects() -> Html {

    let projects: Vec<Project> = {vec![
        Project {
            name: String::from("portfolio"),
            info: String::from("this website"),
            link: String::from(""),
        },
         Project {
            name: String::from("uppy-up"),
            info: String::from("a game where the player moves up"),
            link: String::from(""),
        },
         Project {
            name: String::from("task board api"),
            info: String::from("an example Rust api"),
            link: String::from(""),
        },
    ]};
    
    html! { 
        <div class = "projects">
            <h2> {"Projects"} </h2>
            {
                projects.iter().map(|project| 
                {
                    html!{
                        <ProjectComp project = {(*project).clone()}/>
                    }
                }).collect::<Html>()
            }
        </div> 
    }
}

#[function_component(ProjectComp)]
pub fn project(props: &ProjectCompProps) -> Html {
    html! {
        <div class = "project">
            <h3>{props.project.name.clone()}</h3>
            <p>{props.project.info.clone()}</p>
            <a href = {props.project.link.clone()}>
            </a>
        </div>
    }
}