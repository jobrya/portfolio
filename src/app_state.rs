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

    pub fn icon_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("dark-icon")},
            ModeState::Dark => {String::from("light-icon")},
        }
    }

    pub fn link_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("light-link")},
            ModeState::Dark => {String::from("dark-link")},
        }
    }

    pub fn accent_class(&self) -> String {
        match self {
            ModeState::Light => {String::from("light-accent")},
            ModeState::Dark => {String::from("dark-accent")},
        }
    }

    pub fn home_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/dark_home.png")},
            ModeState::Dark => {String::from("./assets/light_home.png")},
        }
    }

    pub fn mode_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/dark_mode.png")},
            ModeState::Dark => {String::from("./assets/light_mode.png")},
        }
    }

    pub fn folder_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/dark_folder.png")},
            ModeState::Dark => {String::from("./assets/light_folder.png")},
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

    pub fn source_code_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/light_source_code_icon.png")},
            ModeState::Dark => {String::from("./assets/dark_source_code_icon.png")},
        }
    }

    pub fn demo_src(&self) -> String {
        match self {
            ModeState::Light => {String::from("./assets/light_out_icon.png")},
            ModeState::Dark => {String::from("./assets/dark_out_icon.png")},
        }
    }

    pub fn next_state(&self) -> Self {
        match self {
            ModeState::Light => ModeState::Dark,
            ModeState::Dark => ModeState::Light,
        }
    }
}






