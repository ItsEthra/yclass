use crate::{class::ClassList, field::FieldId, process::Process};
use egui_notify::Toasts;

pub struct InspectionContext<'a> {
    pub current_container: usize,
    pub selection: Selection,

    pub address: usize,
    pub offset: usize,
    pub process: &'a Process,
    pub class_list: &'a ClassList,
    pub toasts: &'a mut Toasts,
}

impl InspectionContext<'_> {
    pub fn is_selected(&self, field_id: FieldId) -> bool {
        Some(self.current_container) == self.selection.container
            && self.selection.field == Some(field_id)
    }

    pub fn select(&mut self, field_id: FieldId) {
        if self.selection.field == Some(field_id) {
            self.deselect();
        } else {
            self.selection.field = Some(field_id);
            self.selection.container = Some(self.current_container);
        }
    }

    pub fn deselect(&mut self) {
        self.selection.field = None;
        self.selection.container = None;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Selection {
    pub field: Option<FieldId>,
    pub container: Option<usize>,
}
