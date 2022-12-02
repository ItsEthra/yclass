use eframe::{
    egui::{style::Margin, Context, Frame, TopBottomPanel},
    epaint::Rounding,
};

pub enum ToolBarResponse {
    ToggleAttachWindow,
    ProcessDetach,
}

#[derive(Default)]
pub struct ToolBarPanel;

impl ToolBarPanel {
    pub fn show(&mut self, ctx: &Context) -> Option<ToolBarResponse> {
        let style = ctx.style();
        let frame = Frame {
            inner_margin: Margin::same(0.),
            rounding: Rounding::none(),
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
            ..Default::default()
        };

        let mut response = None;
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
                            response = Some(ToolBarResponse::ToggleAttachWindow);
                            ui.close_menu();
                        }

                        // Reattach to last process
                        // let _ = ui.button("Reattach to process");
                        if ui.button("Detach from process").clicked() {
                            response = Some(ToolBarResponse::ProcessDetach);
                            ui.close_menu();
                        }
                    });
                });
            });

        response
    }
}
