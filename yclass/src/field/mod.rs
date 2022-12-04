mod hex;
use std::cell::{Cell, RefCell};

pub use hex::*;
mod int;
pub use int::*;
mod kind;
pub use kind::*;
mod utils;
pub use utils::*;
mod float;
pub use float::*;
mod pointer;
pub use pointer::*;

use crate::{class::Class, context::InspectionContext, generator::Generator, FID_M};
use eframe::{
    egui::{TextFormat, Ui},
    epaint::{Color32, Stroke},
};

pub type FieldId = u64;

pub enum FieldResponse {
    NewClass(String, usize),
}

pub trait Field {
    fn id(&self) -> FieldId;
    fn name(&self) -> Option<String>;
    fn size(&self) -> usize;

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse>;
    fn codegen(&self, generator: &mut dyn Generator, data: &CodegenData);
}

pub struct CodegenData<'a> {
    pub classes: &'a [Class],
}

#[derive(Default)]
pub struct NamedState {
    editing: Cell<bool>,
    request_focus: Cell<bool>,
    name: RefCell<String>,
    saved_name: RefCell<String>,
}

impl NamedState {
    pub fn new(name: String) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

mod private {
    use super::FieldId;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    pub fn next_id() -> FieldId {
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }
}

pub use private::next_id;

fn create_text_format(selected: bool, col: Color32) -> TextFormat {
    if selected {
        TextFormat {
            underline: Stroke::new(1., Color32::LIGHT_GRAY),
            ..TextFormat::simple(FID_M, col)
        }
    } else {
        TextFormat::simple(FID_M, col)
    }
}
