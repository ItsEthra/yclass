use crate::state::StateRef;
use eframe::{
    egui::{Context, RichText, ScrollArea, TextEdit, Window},
    epaint::{vec2, FontId},
};
use memflex::external::ProcessIterator;

pub struct ProcessAttachWindow {
    shown: bool,
    poisoned: bool,
    filter: String,
    state: StateRef,
}

impl ProcessAttachWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            poisoned: false,
            shown: false,
            filter: "".to_owned(),
            state,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn show(&mut self, ctx: &Context) -> Option<u32> {
        if !self.shown {
            return None;
        }

        let mut attach_pid = None;
        Window::new("Attach to process")
            .collapsible(false)
            .open(&mut self.shown)
            .default_size(vec2(180., 320.))
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    TextEdit::singleline(&mut self.filter)
                        .desired_width(f32::INFINITY)
                        .hint_text("Filter by name")
                        .show(ui);

                    ScrollArea::vertical().show(ui, |ui| match ProcessIterator::new() {
                        Ok(piter) => {
                            for pe in piter.filter(|pe| {
                                self.filter.is_empty()
                                    || pe.name.to_lowercase().contains(&self.filter.to_lowercase())
                            }) {
                                if ui
                                    .button(
                                        RichText::new(format!("{} - {}", pe.name, pe.id))
                                            .font(FontId::proportional(16.)),
                                    )
                                    .clicked()
                                {
                                    attach_pid = Some(pe.id);
                                }
                            }
                        }
                        Err(e) if self.poisoned => {
                            _ = self
                                .state
                                .borrow_mut()
                                .toasts
                                .error(format!("Failed to iterate over processes: {e}"));
                            self.poisoned = true;
                        }
                        _ => {}
                    });
                });
            });

        attach_pid
    }
}
