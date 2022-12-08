use crate::{
    address::parse_address,
    context::{InspectionContext, Selection},
    field::FieldResponse,
    state::StateRef,
    FID_M,
};
use eframe::{
    egui::{collapsing_header::CollapsingState, CentralPanel, Context, Id, ScrollArea, Ui},
    epaint::FontId,
};
use fastrand::Rng;

pub struct InspectorPanel {
    pub selection: Option<Selection>,

    address_buffer: String,
    state: StateRef,
    address: usize,
    allow_scroll: bool,
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
            allow_scroll: true,
            selection: None,
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

    fn inspect(&mut self, ui: &mut Ui) -> Option<()> {
        let state = &mut *self.state.borrow_mut();
        let rng = Rng::with_seed(0);

        let mut ctx = InspectionContext {
            current_container: state.class_list.selected()?,
            process: state.process.as_ref()?,
            class_list: &state.class_list,
            toasts: &mut state.toasts,
            selection: self.selection,
            current_id: Id::new(0),
            address: self.address,
            parent_id: Id::new(0),
            level_rng: &rng,
            offset: 0,
        };

        let class = state.class_list.selected_class()?;

        let mut new_class = None;
        #[allow(clippy::single_match)]
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .enable_scrolling(self.allow_scroll)
            .show(ui, |ui| {
                match class.fields.iter().fold(None, |r, f| {
                    ctx.current_id = Id::new(rng.u64(..));
                    r.or(f.draw(ui, &mut ctx))
                }) {
                    Some(FieldResponse::NewClass(name, id)) => {
                        new_class = Some((name, id));
                    }
                    Some(FieldResponse::LockScroll) => self.allow_scroll = false,
                    Some(FieldResponse::UnlockScroll) => self.allow_scroll = true,
                    None => {}
                }
            })
            .inner;
        self.selection = ctx.selection;

        if let Some((name, id)) = new_class {
            state.class_list.add_class_with_id(name, id);
        }

        Some(())
    }
}
