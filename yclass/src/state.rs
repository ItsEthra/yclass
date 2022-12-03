use crate::class::Class;
use egui_notify::Toasts;
use memflex::external::OwnedProcess;
use std::cell::RefCell;
use yclass_config::YClassConfig;

pub type StateRef = &'static RefCell<GlobalState>;

#[derive(Default)]
pub struct GlobalState {
    pub toasts: Toasts,
    pub process: Option<OwnedProcess>,
    pub class_list: Vec<Class>,
    pub selected_class: Option<usize>,
    pub config: YClassConfig,
}
