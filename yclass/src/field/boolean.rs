use std::slice;

use super::{
    create_text_format, display_field_name, display_field_prelude, next_id, CodegenData, Field,
    FieldId, FieldKind, FieldResponse, NamedState,
};
use crate::{context::InspectionContext, generator::Generator};
use eframe::{
    egui::{Label, Sense, Ui},
    epaint::{text::LayoutJob, Color32},
};

pub struct BoolField {
    id: FieldId,
    state: NamedState,
}

impl BoolField {
    pub fn new(name: String) -> Self {
        Self {
            id: next_id(),
            state: NamedState::new(name),
        }
    }
}

impl Field for BoolField {
    fn id(&self) -> FieldId {
        self.id
    }

    fn name(&self) -> Option<String> {
        Some(self.state.name.borrow().clone())
    }

    fn size(&self) -> usize {
        1
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut val = 0u8;
        ctx.process
            .read(ctx.address + ctx.offset, slice::from_mut(&mut val));

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                ctx.select(self.id);
            }

            display_field_name(self, ui, ctx, &self.state, Color32::GOLD);

            let mut job = LayoutJob::default();
            job.append(
                match val {
                    1 => "true",
                    0 => "false",
                    _ => "invalid",
                },
                0.,
                create_text_format(ctx.is_selected(self.id), Color32::WHITE),
            );

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                ctx.select(self.id);
            }
        });

        ctx.offset += 1;
        None
    }

    fn codegen(&self, generator: &mut dyn Generator, _: &CodegenData) {
        generator.add_field(self.state.name.borrow().as_str(), FieldKind::Bool, None);
    }
}
