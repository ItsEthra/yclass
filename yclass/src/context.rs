use crate::{field::FieldId, process::Process};

pub struct InspectionContext<'a> {
    pub selected: Option<FieldId>,
    pub selected_container: Option<usize>,

    pub address: usize,
    pub offset: usize,
    pub process: &'a Process,
}
