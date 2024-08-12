
#[derive(Default)]
pub struct AppState {
    pub mode: ModeState,
}


#[derive(Clone, Default, PartialEq)]
pub enum ModeState{
    Light,
    #[default]
    Dark,
}

impl ModeState {
    pub fn to_string(&self) -> String {
        match self {
            ModeState::Light => {String::from("light-mode")},
            ModeState::Dark => {String::from("dark-mode")},
        }
    }

    pub fn button_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("dark-button")},
            ModeState::Dark => {String::from("light-button")},
        }
    }

    pub fn contact_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("dark-contact")},
            ModeState::Dark => {String::from("light-contact")},
        }
    }

    pub fn link_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("light-link")},
            ModeState::Dark => {String::from("dark-link")},
        }
    }

    pub fn github_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/github-mark.png")},
            ModeState::Dark => {String::from("./assets/github-mark-white.png")},
        }
    }

    pub fn linkedin_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/In-Blue-34@2x.png")},
            ModeState::Dark => {String::from("./assets/In-White-34@2x.png")},
        }
    }

    pub fn mail_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/mail_light.png")},
            ModeState::Dark => {String::from("./assets/mail_dark.png")},
        }
    }

    pub fn next_state(&self) -> Self {
        match self {
            ModeState::Light => ModeState::Dark,
            ModeState::Dark => ModeState::Light,
        }
    }
}






