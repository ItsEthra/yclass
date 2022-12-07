use crate::{
    context::Selection,
    field::allocate_padding,
    gui::{ClassListPanel, InspectorPanel, ToolBarPanel, ToolBarResponse},
    process::Process,
    state::StateRef,
};
use eframe::{egui::Context, epaint::Color32, App, Frame};
use std::sync::Once;

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
        static DPI_INIT: Once = Once::new();
        DPI_INIT.call_once(|| {
            let dpi = self.state.borrow().config.dpi.unwrap_or(1.);
            ctx.set_pixels_per_point(dpi);
        });

        match self.tool_bar.show(ctx) {
            Some(ToolBarResponse::Add(n)) => {
                let state = &mut *self.state.borrow_mut();

                if let Some(cid) = self
                    .inspector
                    .selection
                    .map(|s| s.container_id)
                    .or_else(|| state.class_list.selected())
                {
                    let class = state.class_list.by_id_mut(cid).unwrap();
                    class.fields.extend(allocate_padding(n));

                    state.dummy = false;
                }
            }
            Some(ToolBarResponse::Remove(n)) => {
                if let Some(Selection {
                    container_id,
                    field_id,
                    ..
                }) = self.inspector.selection
                {
                    let state = &mut *self.state.borrow_mut();

                    let class = state.class_list.by_id_mut(container_id).unwrap();
                    let pos = class
                        .fields
                        .iter()
                        .position(|f| f.id() == field_id)
                        .unwrap();

                    let from = pos.min(class.fields.len());
                    let to = (pos + n).min(class.fields.len());

                    class.fields.drain(from..to);

                    state.dummy = false;
                }
            }
            Some(ToolBarResponse::Insert(n)) => {
                if let Some(Selection {
                    container_id,
                    field_id,
                    ..
                }) = self.inspector.selection
                {
                    let state = &mut *self.state.borrow_mut();

                    let class = state.class_list.by_id_mut(container_id).unwrap();
                    let pos = class
                        .fields
                        .iter()
                        .position(|f| f.id() == field_id)
                        .unwrap();
                    let mut padding = allocate_padding(n);

                    while let Some(field) = padding.pop() {
                        class.fields.insert(pos, field);
                    }

                    state.dummy = false;
                }
            }
            Some(ToolBarResponse::ChangeKind(new)) => {
                if let Some(Selection {
                    container_id,
                    field_id,
                    ..
                }) = self.inspector.selection
                {
                    let state = &mut *self.state.borrow_mut();

                    let class = state.class_list.by_id_mut(container_id).unwrap();
                    let pos = class
                        .fields
                        .iter()
                        .position(|f| f.id() == field_id)
                        .unwrap();

                    let (old_size, old_name) = (class.fields[pos].size(), class.fields[pos].name());
                    if old_size > new.size() {
                        let mut padding = allocate_padding(old_size - new.size());
                        class.fields[pos] = new.into_field(old_name);
                        while let Some(pad) = padding.pop() {
                            class.fields.insert(pos + 1, pad);
                        }

                        self.inspector.selection.as_mut().unwrap().field_id =
                            class.fields[pos].id();
                    } else {
                        let mut steal_size = 0;
                        while steal_size < new.size() {
                            if class.fields.len() <= pos {
                                break;
                            }

                            steal_size += class.fields.remove(pos).size();
                        }

                        let mut padding = allocate_padding(steal_size - new.size());
                        class.fields.insert(pos, new.into_field(old_name));

                        while let Some(pad) = padding.pop() {
                            class.fields.insert(pos + 1, pad);
                        }

                        self.inspector.selection.as_mut().unwrap().field_id =
                            class.fields[pos].id();
                    }

                    state.dummy = false;
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
                            match op.name() {
                                Ok(name) => {
                                    state.config.last_attached_process_name = Some(name);
                                    state.config.save();
                                }
                                Err(e) => {
                                    _ = state
                                        .toasts
                                        .error(format!("Failed to get process name: {e}"))
                                }
                            }
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
