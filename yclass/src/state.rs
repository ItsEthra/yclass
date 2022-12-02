use egui_notify::Toasts;
use std::cell::RefCell;

pub type StateRef = &'static GlobalState;

#[derive(Default)]
pub struct GlobalState {
    pub toasts: RefCell<Toasts>,
}
