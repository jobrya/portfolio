use yew::{function_component, html, Html, Properties};
use crate::app_state::ModeState;

#[derive(Clone, PartialEq)]
struct Contact {
    link: String,
    img_src: String,
    alt: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactProps {
    pub mode: ModeState,
}


#[function_component(Contacts)]
pub fn contacts(props: &ContactProps) -> Html {
    let contacts = vec![
        Contact {
            link: String::from("https://github.com/jobrya"),
            img_src: props.mode.github_src(),
            alt: String::from("github icon"),
        },
        Contact {
            link: String::from("https://www.linkedin.com/in/josh-o-bryant-582160181"),
            img_src: props.mode.linkedin_src(),
            alt: String::from("linkedin icon"),
        },
        Contact {
            link: String::from("mailto:jobryant.dev@gmail.com"),
            img_src: props.mode.mail_src(),
            alt: String::from("mail icon"),
        },
    ];

    html! { 
        <div class = "contacts">
            {
            contacts.iter().map(|contact| 
                {
                    html!{
                        <ContactComp contact = {(*contact).clone()} mode = {props.mode.clone()}/>
                    }
                }).collect::<Html>()
            }
        </div> 
    }
}

#[derive(Properties, PartialEq)]
pub struct ContactCompProps {
    contact: Contact,
    mode: ModeState,
}

#[function_component(ContactComp)]
pub fn contact(props: &ContactCompProps) -> Html {
    html! {
        <div class = "contact">
            <a href = {props.contact.link.clone()} target = "_blank" rel = "noopener">
            <img src = {props.contact.img_src.clone()} alt = {props.contact.alt.clone()} class = {props.mode.contact_class()}/>
            </a>
        </div>
    }
}