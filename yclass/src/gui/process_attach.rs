use eframe::{
    egui::{Context, RichText, ScrollArea, TextEdit, Window},
    epaint::{vec2, FontId},
};
use memflex::external::ProcessIterator;

#[derive(Default)]
pub struct ProcessAttachWindow {
    shown: bool,
    filter: String,
}

impl ProcessAttachWindow {
    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn show(&mut self, ctx: &Context) -> Option<u32> {
        if !self.shown {
            return None;
        }

        let mut attach_pid = None;
        Window::new("Attach")
            .title_bar(false)
            .default_size(vec2(180., 320.))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    TextEdit::singleline(&mut self.filter)
                        .desired_width(f32::INFINITY)
                        .hint_text("Filter by name")
                        .show(ui);

                    ScrollArea::vertical().show(ui, |ui| {
                        for pe in ProcessIterator::new().unwrap().filter(|pe| {
                            self.filter.is_empty()
                                || pe.name.to_lowercase().contains(&self.filter.to_lowercase())
                        }) {
                            #[cfg(unix)]
                            let text = format!("{} - {}", pe.name, pe.id);
                            #[cfg(windows)]
                            let text = format!("{} - 0x{:X}", pe.name, pe.id);

                            if ui
                                .button(RichText::new(text).font(FontId::proportional(16.)))
                                .clicked()
                            {
                                attach_pid = Some(pe.id);
                            }
                        }
                    });
                });
            });

        attach_pid
    }
}
