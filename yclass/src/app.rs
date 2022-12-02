use crate::gui::{ClassListPanel, ProcessAttachWindow, ToolBarPanel, ToolBarResponse};
use eframe::{
    egui::{CentralPanel, Context},
    App, Frame,
};

#[derive(Default)]
pub struct YClassApp {
    ps_attach_window: ProcessAttachWindow,
    class_list: ClassListPanel,
    tool_bar: ToolBarPanel,
}

impl App for YClassApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if let Some(ToolBarResponse::ToggleAttachWindow) = self.tool_bar.show(ctx) {
            self.ps_attach_window.toggle();
        }

        self.class_list.show(ctx);
        self.ps_attach_window.show(ctx);

        CentralPanel::default().show(ctx, |_ui| {});
    }
}
