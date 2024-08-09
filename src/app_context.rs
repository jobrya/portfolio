use std::rc::Rc;
use yew::prelude::*;
use gloo_console::log;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

impl Mode {
    pub fn to_string(&self) -> String {
        match self {
            Mode::Dark => {String::from("dark-mode")},
            Mode::Light => {String::from("light-mode")},
        }
    }
}

pub enum ModeAction {
    Switch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub mode: UseReducerHandle<Mode>,
}

impl Reducible for Mode {
    type Action = ModeAction;

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        match *self {
            Mode::Dark => {
                log!("mode switching to light");
                return Mode::Light.into();
            },
            Mode::Light => {
               log!("mode switching to dark");
                return Mode::Dark.into()
            },
        }
    }
}