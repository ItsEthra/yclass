use crate::{
    address::parse_address,
    context::InspectionContext,
    field::{FieldId, FieldResponse},
    state::StateRef,
    FID_M,
};
use eframe::{
    egui::{collapsing_header::CollapsingState, CentralPanel, Context, Id, ScrollArea, Ui},
    epaint::FontId,
};

pub struct InspectorPanel {
    selected_container: Option<usize>,
    selected: Option<FieldId>,
    address_buffer: String,
    state: StateRef,
    address: usize,
}

impl InspectorPanel {
    pub fn new(state: StateRef) -> Self {
        #[cfg(not(debug_assertions))]
        let address = 0;
        #[cfg(debug_assertions)]
        let address = state.borrow().config.last_address.unwrap_or(0);

        Self {
            state,
            address,
            selected: None,
            selected_container: None,
            address_buffer: format!("0x{address:X}"),
        }
    }

    pub fn show(&mut self, ctx: &Context) -> Option<()> {
        CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.style_mut().override_font_id = Some(FontId::monospace(18.));

                let state = self.state.borrow();
                if state.process.is_none() || state.class_list.selected_class().is_none() {
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
                                self.address = addr;
                                #[cfg(debug_assertions)]
                                {
                                    state.config.last_address = Some(addr);
                                    state.config.save();
                                }
                            } else {
                                state.toasts.error("Address is in invalid format");
                            }
                            self.address_buffer = format!("0x{:X}", self.address);
                        }

                        Some(())
                    })
                    .body(|ui| self.inspect(ui));
            });
        });

        None
    }

    pub fn selected_container(&self) -> Option<usize> {
        self.selected_container
    }

    pub fn selected_field(&self) -> Option<FieldId> {
        self.selected
    }

    pub fn set_selected_field(&mut self, id: Option<FieldId>) {
        self.selected = id;
    }

    fn inspect(&mut self, ui: &mut Ui) -> Option<()> {
        let state = &*self.state.borrow_mut();
        let mut ctx = InspectionContext {
            selected_container: self.selected_container.or(state.class_list.selected()),
            process: state.process.as_ref()?,
            selected: self.selected,
            address: self.address,
            offset: 0,
        };

        let class = state.class_list.selected_class()?;

        #[allow(clippy::single_match)]
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                match class
                    .fields
                    .iter()
                    .fold(None, |r, f| r.or(f.draw(ui, &mut ctx)))
                {
                    Some(FieldResponse::Selected(sid)) => {
                        self.selected = Some(sid);
                    }
                    None => {}
                }
            });
        self.selected_container = ctx.selected_container;

        Some(())
    }
}
