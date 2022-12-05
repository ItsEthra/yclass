use crate::{
    context::Selection,
    field::{allocate_padding, Field, HexField},
    gui::{ClassListPanel, InspectorPanel, ToolBarPanel, ToolBarResponse},
    process::Process,
    state::StateRef,
};
use eframe::{egui::Context, epaint::Color32, App, Frame};

pub struct YClassApp {
    class_list: ClassListPanel,
    inspector: InspectorPanel,
    tool_bar: ToolBarPanel,
    state: StateRef,
}

impl YClassApp {
    pub fn new(state: StateRef) -> Self {
        Self {
            class_list: ClassListPanel::new(state),
            inspector: InspectorPanel::new(state),
            tool_bar: ToolBarPanel::new(state),
            state,
        }
    }
}

impl App for YClassApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match self.tool_bar.show(ctx) {
            Some(ToolBarResponse::Add(n)) => {
                let state = &mut *self.state.borrow_mut();
                if let Some(cid) = self
                    .inspector
                    .selection()
                    .container
                    .or(state.class_list.selected())
                {
                    let class = state.class_list.by_id_mut(cid).unwrap();
                    class.fields.extend(allocate_padding(n));
                }
            }
            Some(ToolBarResponse::Remove(n)) => {
                if let Selection {
                    field: Some(field_id),
                    container: Some(container_id),
                } = self.inspector.selection()
                {
                    let state = &mut *self.state.borrow_mut();
                    let class = state.class_list.by_id_mut(container_id).unwrap();

                    if let Some(pos) = class.fields.iter().position(|f| f.id() == field_id) {
                        class
                            .fields
                            .drain(pos.min(class.fields.len())..(pos + n).min(class.fields.len()));

                        if let Some(new_selection) = class.fields.get(pos).map(|f| f.id()) {
                            self.inspector.selection_mut().field = Some(new_selection);
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Some(ToolBarResponse::ChangeKind(new)) => {
                if let Selection {
                    field: Some(field_id),
                    container: Some(container_id),
                } = self.inspector.selection()
                {
                    let state = &mut *self.state.borrow_mut();
                    let class = state.class_list.by_id_mut(container_id).unwrap();

                    if let Some(pos) = class.fields.iter().position(|f| f.id() == field_id) {
                        let old_size = class.fields[pos].size();
                        let replacement = new.into_field(class.fields[pos].name());

                        if old_size > new.size() {
                            let rest = old_size - new.size();

                            self.inspector.selection_mut().field = Some(replacement.id());
                            class.fields[pos] = replacement;

                            let mut fill = allocate_padding(rest);
                            while let Some(f) = fill.pop() {
                                class.fields.insert(pos + 1, f);
                            }
                        } else {
                            let mut stolen = 0;
                            while stolen < new.size() {
                                stolen += class.fields.remove(pos).size();
                            }

                            self.inspector.selection_mut().field = Some(replacement.id());
                            class.fields.insert(pos, replacement);

                            let mut fill = allocate_padding(stolen - new.size());
                            while let Some(f) = fill.pop() {
                                class.fields.insert(pos + 1, f);
                            }
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            Some(ToolBarResponse::ProcessDetach) => {
                self.state.borrow_mut().process = None;
                frame.set_window_title("YClass");
            }
            Some(ToolBarResponse::ProcessAttach(pid)) => {
                let state = &mut *self.state.borrow_mut();
                match Process::attach(pid, &state.config) {
                    Ok(proc) => {
                        frame.set_window_title(&format!("YClass - Attached to {pid}"));
                        if let Process::Internal((op, _)) = &proc {
                            state.config.last_attached_process_name = Some(op.name());
                            state.config.save();
                        }

                        state.process = Some(proc);
                    }
                    Err(e) => {
                        state.toasts.error(format!(
                            "Failed to attach to process.\nPossibly plugin error.\n{e}"
                        ));
                    }
                }
            }
            None => {}
        }

        self.class_list.show(ctx);
        self.inspector.show(ctx);

        let mut style = (*ctx.style()).clone();
        let saved = style.clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x10, 0x10, 0x10);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::LIGHT_GRAY;
        ctx.set_style(style);

        self.state.borrow_mut().toasts.show(ctx);
        ctx.set_style(saved);
    }
}

pub fn is_valid_ident(name: &str) -> bool {
    !name.starts_with(char::is_numeric) && !name.contains(char::is_whitespace) && !name.is_empty()
}
