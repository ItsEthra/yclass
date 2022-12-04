use crate::{app::is_valid_ident, state::StateRef};
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
                ui.set_enabled(state.class_list.selected().is_some());

                let w = ui.available_width();
                if ui.add_sized(vec2(w, 18.), Button::new("Rename")).clicked() {
                    self.edit_state = Some((
                        state.class_list.selected_class().unwrap().name.clone(),
                        false,
                        state.class_list.selected().unwrap(),
                    ));
                }

                if ui.add_sized(vec2(w, 18.), Button::new("Delete")).clicked() {
                    state
                        .class_list
                        .delete_by_id(state.class_list.selected().unwrap());
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
                    .classes()
                    .iter()
                    .any(|c| c.name == self.new_class_buf)
                {
                    state
                        .toasts
                        .error("Class with the same name already exists");
                } else if !is_valid_ident(&self.new_class_buf) {
                    state.toasts.error("Not a valid class name");
                } else {
                    state.class_list.add_class(take(&mut self.new_class_buf));
                }
            }

            ScrollArea::vertical().show(ui, |ui| {
                let w = ui.available_width();

                let selected = state.class_list.selected();
                let mut new_selection = None;
                for class in state.class_list.classes_mut() {
                    if let Some((edit_buf, focused)) =
                        self.edit_state.as_mut().and_then(|(buf, focused, j)| {
                            if *j == class.id() {
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
                    } else {
                        let r = ui.add_sized(
                            vec2(w, 18.),
                            SelectableLabel::new(
                                selected.map(|j| class.id() == j).unwrap_or_default(),
                                &class.name,
                            ),
                        );

                        if r.secondary_clicked() {
                            self.edit_state = Some((class.name.clone(), false, class.id()));
                        } else if r.clicked() {
                            if selected == Some(class.id()) {
                                new_selection = Some(None);
                            } else {
                                new_selection = Some(Some(class.id()));
                            }
                        }
                    }
                }

                if let Some(new) = new_selection {
                    *state.class_list.selected_mut() = new;
                }
            });
        });
    }
}
