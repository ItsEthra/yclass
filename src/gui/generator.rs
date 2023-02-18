use crate::{field::CodegenData, generator::AvailableGenerator, state::StateRef};
use eframe::{
    egui::{ComboBox, Context, FontSelection, TextEdit, Window},
    epaint::FontId,
};

pub struct GeneratorWindow {
    current_generator: AvailableGenerator,
    state: StateRef,
    shown: bool,
    output: Option<String>,
}

impl GeneratorWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            shown: false,
            output: None,
            current_generator: AvailableGenerator::default(),
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn show(&mut self, ctx: &Context) {
        if !self.shown {
            return;
        }

        Window::new("Class generator")
            .open(&mut self.shown)
            .show(ctx, |ui| {
                ComboBox::new("_generator", "Current generator")
                    .selected_text(self.current_generator.label())
                    .show_ui(ui, |ui| {
                        for gen in AvailableGenerator::ALL {
                            if ui
                                .selectable_label(self.current_generator == *gen, gen.label())
                                .clicked()
                            {
                                self.current_generator = *gen;
                            }
                        }
                    });

                ui.horizontal(|ui| {
                    if ui.button("Generate").clicked() {
                        let mut gen = self.current_generator.generator();
                        let state = self.state.borrow();
                        let data = CodegenData {
                            classes: state.class_list.classes(),
                        };

                        for class in state.class_list.classes() {
                            gen.begin_class(&class.name);
                            for field in class.fields.iter() {
                                field.codegen(&mut *gen, &data);
                            }
                            gen.end_class();
                        }
                        self.output = Some(gen.finilize());
                    }

                    if let Some(ref out) = self.output {
                        if ui.button("Copy to clipboard").clicked() {
                            ui.ctx().output_mut(|o| o.copied_text = out.clone());
                            self.state
                                .borrow_mut()
                                .toasts
                                .info("Output was copied to clipboard");
                        }
                    }
                });

                if let Some(ref out) = self.output {
                    TextEdit::multiline(&mut out.as_str())
                        .font(FontSelection::FontId(FontId::monospace(12.)))
                        .show(ui);
                }
            });
    }
}
