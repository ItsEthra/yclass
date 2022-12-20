use crate::{app::is_valid_ident, state::StateRef};
use eframe::{
    egui::{Context, Key, ScrollArea, SelectableLabel, SidePanel, TextEdit},
    epaint::vec2,
};
use std::mem::take;

struct ClassEditState {
    request_focus: bool,
    new_name: String,
    id: usize,
}

pub struct ClassListPanel {
    new_class_buf: String,
    edit_state: Option<ClassEditState>,
    should_focus_edit: bool,
    state: StateRef,
}

impl ClassListPanel {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            edit_state: None,
            should_focus_edit: false,
            new_class_buf: "".to_owned(),
        }
    }

    pub fn show(&mut self, ctx: &Context) {
        SidePanel::left("_class_list").show(ctx, |ui| {
            ui.add_space(4.);
            ui.vertical_centered_justified(|ui| {
                ui.heading("Class list")
                    .on_hover_text("Press ENTER to create a new class");
            });
            ui.add_space(4.);

            let state = &mut *self.state.borrow_mut();
            let r = TextEdit::singleline(&mut self.new_class_buf)
                .desired_width(f32::INFINITY)
                .hint_text("Create new class")
                .show(ui)
                .response;

            if self.should_focus_edit {
                r.request_focus();
                self.should_focus_edit = false;
            }

            ui.add_space(4.);
            ui.separator();
            ui.add_space(4.);

            if r.clicked_elsewhere() || (ui.input().key_pressed(Key::Escape) && r.lost_focus()) {
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
                    self.should_focus_edit = true;
                } else {
                    state.class_list.add_class(take(&mut self.new_class_buf));
                    state.dummy = false;
                }
            }

            ui.vertical_centered_justified(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let selected = state.class_list.selected();
                    let mut clicked_class = None;
                    let mut delete_class = None;

                    for class in state.class_list.classes_mut() {
                        if let Some((edit_buf, request_focus)) = self.edit_state.as_mut().and_then(
                            |ClassEditState {
                                 id,
                                 new_name,
                                 request_focus,
                             }| {
                                if *id == class.id() {
                                    Some((new_name, request_focus))
                                } else {
                                    None
                                }
                            },
                        ) {
                            let r = TextEdit::singleline(edit_buf)
                                .desired_width(f32::INFINITY)
                                .hint_text("New name")
                                .show(ui)
                                .response;

                            let first_frame = if *request_focus {
                                r.request_focus();
                                *request_focus = false;
                                true
                            } else {
                                false
                            };

                            if r.clicked_elsewhere() && !first_frame {
                                self.edit_state = None;
                            } else if r.lost_focus() {
                                if !is_valid_ident(&*edit_buf) {
                                    state.toasts.error("Not a valid class name");
                                    *request_focus = true;
                                } else {
                                    class.name = take(edit_buf);
                                    self.edit_state = None;
                                    state.dummy = false;
                                }
                            }
                        } else {
                            let r = ui.add_sized(
                                vec2(ui.available_width(), 24.),
                                SelectableLabel::new(
                                    selected.map(|j| class.id() == j).unwrap_or_default(),
                                    &class.name,
                                ),
                            );

                            if r.clicked() {
                                clicked_class = Some(class.id());
                            }

                            r.context_menu(|ui| {
                                ui.set_width(80.);

                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("Rename").clicked() {
                                        ui.close_menu();

                                        self.edit_state = Some(ClassEditState {
                                            new_name: class.name.clone(),
                                            request_focus: true,
                                            id: class.id(),
                                        });
                                    }

                                    if ui.button("Delete").clicked() {
                                        ui.close_menu();

                                        delete_class = Some(class.id());
                                    }
                                });
                            });
                        }
                    }

                    if let Some(delete) = delete_class {
                        state.class_list.delete_by_id(delete);
                    }

                    if let Some(new) = clicked_class {
                        let selected = state.class_list.selected_mut();
                        if *selected == Some(new) {
                            *selected = None;
                        } else {
                            *selected = Some(new);
                        }
                    }
                });
            });
        });
    }
}
