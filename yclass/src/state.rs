use crate::{class::ClassList, config::YClassConfig, process::Process};
use egui_notify::Toasts;
use std::{cell::RefCell, path::PathBuf};

pub type StateRef = &'static RefCell<GlobalState>;

pub struct GlobalState {
    pub toasts: Toasts,
    pub process: Option<Process>,
    pub class_list: ClassList,
    pub config: YClassConfig,
    pub last_opened_project: Option<PathBuf>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            toasts: Toasts::default(),
            last_opened_project: None,
            process: None,
            class_list: ClassList::default(),
            config: YClassConfig::load_or_default(),
        }
    }
}
