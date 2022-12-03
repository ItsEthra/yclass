use crate::{
    gui::{ClassListPanel, InspectorPanel, ToolBarPanel, ToolBarResponse},
    state::StateRef,
};
use eframe::{egui::Context, epaint::Color32, App, Frame};
use memflex::external::find_process_by_id;

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
            Some(ToolBarResponse::ProcessDetach) => {
                self.state.borrow_mut().process = None;
                frame.set_window_title("YClass");
            }
            Some(ToolBarResponse::ProcessAttach(pid)) => {
                let state = &mut *self.state.borrow_mut();
                match find_process_by_id(pid) {
                    Ok(proc) => {
                        frame.set_window_title(&format!("YClass - Attached to {}", proc.name()));

                        state.config.last_attached_process_name = Some(proc.name());
                        state.config.save();
                        state.process = Some(proc);
                    }
                    Err(e) => {
                        state
                            .toasts
                            .error(format!("Failed to attach to process: {e}"));
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
