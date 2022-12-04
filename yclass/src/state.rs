use crate::{class::ClassList, config::YClassConfig, process::Process};
use egui_notify::Toasts;
use std::cell::RefCell;

pub type StateRef = &'static RefCell<GlobalState>;

pub struct GlobalState {
    pub toasts: Toasts,
    pub process: Option<Process>,
    pub class_list: ClassList,
    pub config: YClassConfig,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            toasts: Toasts::default(),
            process: None,
            class_list: ClassList::default(),
            config: YClassConfig::load_or_default(),
        }
    }
}
