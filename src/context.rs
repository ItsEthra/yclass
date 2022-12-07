use crate::{
    class::{ClassId, ClassList},
    field::FieldId,
    process::Process,
};
use egui_notify::Toasts;

pub struct InspectionContext<'a> {
    pub selection: Option<Selection>,
    pub current_container: usize,

    pub address: usize,
    pub offset: usize,

    pub process: &'a Process,
    pub class_list: &'a ClassList,
    pub toasts: &'a mut Toasts,
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub address: usize,
    pub container_id: ClassId,
    pub field_id: FieldId,
}

impl InspectionContext<'_> {
    pub fn select(&mut self, field_id: FieldId) {
        if self.is_selected(field_id) {
            self.selection = None;
        } else {
            self.selection = Some(Selection {
                container_id: self.current_container,
                address: self.address + self.offset,
                field_id,
            });
        }
    }

    pub fn is_selected(&self, field_id: FieldId) -> bool {
        self.selection
            .as_ref()
            .map(|s| s.address == self.address + self.offset)
            .unwrap_or(false)
            && self
                .selection
                .as_ref()
                .map(|s| s.field_id == field_id)
                .unwrap_or(false)
    }
}
