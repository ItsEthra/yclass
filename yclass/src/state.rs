use egui_notify::Toasts;
use memflex::external::OwnedProcess;
use std::cell::RefCell;

pub type StateRef = &'static RefCell<GlobalState>;

#[derive(Default)]
pub struct GlobalState {
    pub toasts: Toasts,
    pub process: Option<OwnedProcess>,
}
