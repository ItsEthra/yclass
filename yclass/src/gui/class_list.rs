use eframe::egui::{Context, SidePanel};

pub enum ClassListResponse {}

#[derive(Default)]
pub struct ClassListPanel {}

impl ClassListPanel {
    pub fn show(&mut self, ctx: &Context) -> Option<ClassListResponse> {
        SidePanel::left("_class_list").show(ctx, |_ui| {});

        None
    }
}
