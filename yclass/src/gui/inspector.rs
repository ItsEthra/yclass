use crate::state::StateRef;
use eframe::egui::{CentralPanel, Context};

pub struct InspectorPanel {
    state: StateRef,
}

impl InspectorPanel {
    pub fn new(state: StateRef) -> Self {
        Self { state }
    }

    pub fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |_ui| {});
    }
}
