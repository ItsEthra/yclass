use crate::{
    address::parse_address, context::InspectionContext, field::FieldResponse, state::StateRef,
    FID_M,
};
use eframe::{
    egui::{collapsing_header::CollapsingState, CentralPanel, Context, Id, ScrollArea, Ui},
    epaint::FontId,
};
use fastrand::Rng;

pub struct InspectorPanel {
    address_buffer: String,
    state: StateRef,
    allow_scroll: bool,
}

impl InspectorPanel {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            allow_scroll: true,
            address_buffer: format!("0x{:X}", state.borrow().inspect_address),
        }
    }

    pub fn show(&mut self, ctx: &Context) -> Option<()> {
        CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.style_mut().override_font_id = Some(FontId::monospace(16.));

                let state = self.state.borrow();
                if state.process.is_none() {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Attach to a process to begin inspection.");
                    });
                    return;
                }
                if state.class_list.selected_class().is_none() {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Select a class from the class list to begin inspection.");
                    });
                    return;
                }
                drop(state);

                CollapsingState::load_with_default_open(ctx, Id::new("_inspector_panel"), true)
                    .show_header(ui, |ui| {
                        let state = &mut *self.state.borrow_mut();
                        let active_class = state.class_list.selected_class()?;

                        ui.label(format!("{} - ", active_class.name));

                        ui.spacing_mut().text_edit_width = self
                            .address_buffer
                            .chars()
                            .map(|c| ui.fonts().glyph_width(&FID_M, c))
                            .sum::<f32>()
                            .max(160.);

                        let r = ui.text_edit_singleline(&mut self.address_buffer);
                        if r.lost_focus() {
                            if let Some(addr) = parse_address(&self.address_buffer) {
                                state.inspect_address = addr;
                                #[cfg(debug_assertions)]
                                {
                                    state.config.last_address = Some(addr);
                                    state.config.save();
                                }
                            } else {
                                state.toasts.error("Address is in invalid format");
                            }
                            self.address_buffer = format!("0x{:X}", state.inspect_address);
                        }

                        Some(())
                    })
                    .body(|ui| self.inspect(ui));
            });
        });

        None
    }

    fn inspect(&mut self, ui: &mut Ui) -> Option<()> {
        let state = &mut *self.state.borrow_mut();
        let rng = Rng::with_seed(0);

        let mut ctx = InspectionContext {
            current_container: state.class_list.selected()?,
            process: state.process.as_ref()?,
            address: state.inspect_address,
            class_list: &state.class_list,
            selection: state.selection,
            toasts: &mut state.toasts,
            current_id: Id::new(0),
            parent_id: Id::new(0),
            level_rng: &rng,
            offset: 0,
        };

        let class = state.class_list.selected_class()?;

        let mut new_class = None;
        #[allow(clippy::single_match)]
        ScrollArea::vertical()
            .auto_shrink([false, true])
            .hscroll(true)
            .enable_scrolling(self.allow_scroll)
            .show(ui, |ui| {
                match class.fields.iter().fold(None, |r, f| {
                    ctx.current_id = Id::new(rng.u64(..));
                    r.or(f.draw(ui, &mut ctx))
                }) {
                    Some(FieldResponse::NewClass(name, id)) => new_class = Some((name, id)),
                    Some(FieldResponse::LockScroll) => self.allow_scroll = false,
                    Some(FieldResponse::UnlockScroll) => self.allow_scroll = true,
                    None => {}
                }
            });
        state.selection = ctx.selection;

        if let Some((name, id)) = new_class {
            state.class_list.add_class_with_id(name, id);
        }

        Some(())
    }
}
