use crate::{
    gui::{ClassListPanel, ProcessAttachWindow, ToolBarPanel, ToolBarResponse},
    state::StateRef,
};
use eframe::{
    egui::{CentralPanel, Context},
    epaint::Color32,
    App, Frame,
};

pub struct YClassApp {
    ps_attach_window: ProcessAttachWindow,
    class_list: ClassListPanel,
    tool_bar: ToolBarPanel,
    state: StateRef,
}

impl YClassApp {
    pub fn new(state: StateRef) -> Self {
        Self {
            ps_attach_window: ProcessAttachWindow::new(state),
            class_list: ClassListPanel::default(),
            tool_bar: ToolBarPanel::default(),
            state,
        }
    }
}

impl App for YClassApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match self.tool_bar.show(ctx) {
            Some(ToolBarResponse::ToggleAttachWindow) => {
                self.ps_attach_window.toggle();
            }
            Some(ToolBarResponse::ProcessDetach) => {
                frame.set_window_title("YClass");
            }
            None => {}
        }
        self.class_list.show(ctx);

        if let Some(pid) = self.ps_attach_window.show(ctx) {
            #[cfg(unix)]
            let proc = memflex::external::find_process_by_id(pid);
            #[cfg(windows)]
            let proc = {
                memflex::external::open_process_by_id(pid, false);
            };

            match proc {
                Ok(p) => {
                    frame.set_window_title(&format!("YClass - Attached to {}", p.name()));

                    let state = &mut *self.state.borrow_mut();
                    state.process = Some(p);
                }
                Err(e) => {
                    _ = self
                        .state
                        .borrow_mut()
                        .toasts
                        .error(format!("Failed to attach to the process. {e}"))
                }
            }

            self.ps_attach_window.toggle();
        }

        CentralPanel::default().show(ctx, |_ui| {});

        let mut style = (*ctx.style()).clone();
        let saved = style.clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x10, 0x10, 0x10);
        style.visuals.widgets.noninteractive.fg_stroke.color = Color32::LIGHT_GRAY;
        ctx.set_style(style);

        self.state.borrow_mut().toasts.show(ctx);
        ctx.set_style(saved);
    }
}
