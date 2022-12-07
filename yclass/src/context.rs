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
    pub address: Option<usize>,
    pub container_id: ClassId,
    pub field_id: FieldId,
}

impl InspectionContext<'_> {
    pub fn select(&mut self, field_id: FieldId) {
        self.selection = Some(Selection {
            address: Some(self.address + self.offset),
            container_id: self.current_container,
            field_id,
        });
    }

    pub fn is_selected(&self, field_id: FieldId) -> bool {
        self.selection
            .as_ref()
            .map(|s| {
                s.address
                    .map(|addr| addr == self.address + self.offset)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
            && self
                .selection
                .as_ref()
                .map(|s| s.field_id == field_id)
                .unwrap_or(false)
    }
}
