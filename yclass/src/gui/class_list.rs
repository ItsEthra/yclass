use crate::{class::Class, state::StateRef};
use eframe::{
    egui::{Button, Context, ScrollArea, SelectableLabel, SidePanel, TextEdit},
    epaint::vec2,
};
use std::mem::take;

pub struct ClassListPanel {
    new_class_buf: String,
    edit_state: Option<(String, bool, usize)>,
    state: StateRef,
}

impl ClassListPanel {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            edit_state: None,
            new_class_buf: "".to_owned(),
        }
    }

    pub fn show(&mut self, ctx: &Context) {
        SidePanel::left("_class_list").show(ctx, |ui| {
            ui.add_space(8.);

            let state = &mut *self.state.borrow_mut();
            let r = TextEdit::singleline(&mut self.new_class_buf)
                .desired_width(f32::INFINITY)
                .hint_text("Create new class")
                .show(ui)
                .response;

            ui.vertical_centered_justified(|ui| {
                ui.set_enabled(state.selected_class.is_some());

                let w = ui.available_width();
                if ui.add_sized(vec2(w, 18.), Button::new("Rename")).clicked() {
                    let i = state.selected_class.unwrap();
                    self.edit_state = Some((state.class_list[i].name.clone(), false, i));
                }

                if ui.add_sized(vec2(w, 18.), Button::new("Delete")).clicked() {
                    state
                        .class_list
                        .remove(state.selected_class.take().unwrap());
                }
            });

            ui.add_space(4.);
            ui.separator();
            ui.add_space(4.);

            if r.clicked_elsewhere() {
                self.new_class_buf.clear();
            } else if r.lost_focus() && !self.new_class_buf.is_empty() {
                if state
                    .class_list
                    .iter()
                    .any(|c| c.name == self.new_class_buf)
                {
                    state
                        .toasts
                        .error("Class with the same name already exists");
                } else if !is_valid_ident(&self.new_class_buf) {
                    state.toasts.error("Not a valid class name");
                } else {
                    state
                        .class_list
                        .push(Class::new(take(&mut self.new_class_buf)));
                }
            }

            ScrollArea::vertical().show(ui, |ui| {
                let w = ui.available_width();

                for (i, class) in state.class_list.iter_mut().enumerate() {
                    if let Some((edit_buf, focused)) =
                        self.edit_state.as_mut().and_then(|(buf, focused, j)| {
                            if *j == i {
                                Some((buf, focused))
                            } else {
                                None
                            }
                        })
                    {
                        let r = TextEdit::singleline(edit_buf)
                            .desired_width(f32::INFINITY)
                            .hint_text("New name")
                            .show(ui)
                            .response;

                        let first_frame = if !*focused {
                            r.request_focus();
                            *focused = true;
                            true
                        } else {
                            false
                        };

                        if r.clicked_elsewhere() && !first_frame {
                            self.edit_state = None;
                        } else if r.lost_focus() {
                            if !is_valid_ident(&*edit_buf) {
                                state.toasts.error("Not a valid class name");
                                *focused = false;
                            } else {
                                class.name = take(edit_buf);
                                self.edit_state = None;
                            }
                        }
                    } else if ui
                        .add_sized(
                            vec2(w, 18.),
                            SelectableLabel::new(
                                state.selected_class.map(|j| i == j).unwrap_or_default(),
                                &class.name,
                            ),
                        )
                        .clicked()
                    {
                        if state.selected_class == Some(i) {
                            state.selected_class = None;
                        } else {
                            state.selected_class = Some(i);
                        }
                    }
                }
            });
        });
    }
}

fn is_valid_ident(name: &str) -> bool {
    !name.starts_with(char::is_numeric) && !name.contains(char::is_whitespace) && !name.is_empty()
}
