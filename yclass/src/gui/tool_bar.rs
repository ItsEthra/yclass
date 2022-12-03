use super::ProcessAttachWindow;
use crate::state::StateRef;
use eframe::{
    egui::{style::Margin, Context, Frame, TopBottomPanel},
    epaint::Rounding,
};
use memflex::external::ProcessIterator;

pub enum ToolBarResponse {
    ProcessAttach(u32),
    ProcessDetach,
}

pub struct ToolBarPanel {
    ps_attach_window: ProcessAttachWindow,
    state: StateRef,
}

impl ToolBarPanel {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            ps_attach_window: ProcessAttachWindow::new(state),
        }
    }

    pub fn show(&mut self, ctx: &Context) -> Option<ToolBarResponse> {
        let mut response = None;

        if let Some(pid) = self.ps_attach_window.show(ctx) {
            response = Some(ToolBarResponse::ProcessAttach(pid));
            self.ps_attach_window.toggle();
        }

        let style = ctx.style();
        let frame = Frame {
            inner_margin: Margin::same(0.),
            rounding: Rounding::none(),
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        };

        TopBottomPanel::top("_top_bar")
            .frame(frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.;

                    ui.menu_button("Project", |ui| {
                        let _ = ui.button("New project");
                        let _ = ui.button("Open project");
                        let _ = ui.button("Save project");
                        let _ = ui.button("Save project as");
                    });
                    ui.menu_button("Process", |ui| {
                        if ui.button("Attach to process").clicked() {
                            self.ps_attach_window.toggle();
                            ui.close_menu();
                        }

                        let state = &mut *self.state.borrow_mut();

                        // Reattach to last process
                        if let Some(last_proc_name) =
                            state.config.last_attached_process_name.as_ref().cloned()
                        {
                            if ui.button(format!("Attach to {last_proc_name}")).clicked() {
                                let last_proc = match ProcessIterator::new() {
                                    Ok(mut piter) => piter
                                        .find(|pe| pe.name.eq_ignore_ascii_case(&last_proc_name)),
                                    Err(e) => {
                                        state.toasts.error(format!(
                                            "Failed to iterate over processes. {e}"
                                        ));
                                        return;
                                    }
                                };

                                if let Some(pe) = last_proc {
                                    response = Some(ToolBarResponse::ProcessAttach(pe.id));
                                } else {
                                    state
                                        .toasts
                                        .error(format!("Failed to find {last_proc_name}"));
                                }

                                ui.close_menu();
                            }
                        }

                        if ui.button("Detach from process").clicked() {
                            response = Some(ToolBarResponse::ProcessDetach);
                            ui.close_menu();
                        }
                    });

                    ui.add_space(4.);
                    ui.separator();
                    ui.add_space(4.);

                    if let Some((proc_name, proc_id)) = self
                        .state
                        .borrow()
                        .process
                        .as_ref()
                        .map(|p| (p.name(), p.id()))
                    {
                        #[cfg(unix)]
                        let text = format!("Status: Attached to {} - {}", proc_name, proc_id);
                        #[cfg(windows)]
                        let text = format!("Status: Attached to {} - 0x{:X}", proc_name, proc_id);

                        ui.label(text);
                    } else {
                        ui.label("Status: Detached");
                    }
                });
            });

        response
    }
}
