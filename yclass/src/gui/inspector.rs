use super::FID_M;
use crate::{address::parse_address, state::StateRef};
use eframe::{
    egui::{collapsing_header::CollapsingState, CentralPanel, Context, Id},
    epaint::FontId,
};

pub struct InspectorPanel {
    state: StateRef,
    address_buffer: String,
    address: usize,
}

impl InspectorPanel {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            address: 0,
            address_buffer: "0x0".into(),
        }
    }

    pub fn show(&mut self, ctx: &Context) -> Option<()> {
        let state = &mut *self.state.borrow_mut();
        let active_class = state.selected_class.map(|i| &state.class_list[i])?;

        CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.style_mut().override_font_id = Some(FontId::monospace(18.));

                CollapsingState::load_with_default_open(ctx, Id::new("_inspector_panel"), false)
                    .show_header(ui, |ui| {
                        ui.label(format!("{} - ", active_class.name()));

                        ui.spacing_mut().text_edit_width = self
                            .address_buffer
                            .chars()
                            .map(|c| ui.fonts().glyph_width(&FID_M, c))
                            .sum::<f32>()
                            .max(160.);

                        let r = ui.text_edit_singleline(&mut self.address_buffer);
                        if r.lost_focus() {
                            if let Some(addr) = parse_address(&self.address_buffer) {
                                self.address = addr;
                            } else {
                                state.toasts.error("Address is in invalid format");
                            }
                            self.address_buffer = format!("0x{:X}", self.address);
                        }
                    })
                    .body(|_ui| {});
            });
        });

        None
    }
}
