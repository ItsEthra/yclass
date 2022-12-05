use super::{
    display_field_name, display_field_prelude, display_field_value, next_id, CodegenData, Field,
    FieldId, FieldKind, FieldResponse, NamedState,
};
use crate::{context::InspectionContext, generator::Generator};
use eframe::{
    egui::{Label, Sense, Ui},
    epaint::{text::LayoutJob, Color32},
};

pub struct FloatField<const N: usize> {
    id: FieldId,
    state: NamedState,
}

impl<const N: usize> FloatField<N> {
    pub fn new(name: String) -> Self {
        Self {
            id: next_id(),
            state: NamedState::new(name),
        }
    }
}

impl<const N: usize> Field for FloatField<N> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn size(&self) -> usize {
        N
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut buf = [0; N];
        let address = ctx.address + ctx.offset;
        ctx.process.read(address, &mut buf);

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                ctx.select(self.id);
            }

            display_field_name(self, ui, ctx, &self.state, Color32::LIGHT_RED);
            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                || match N {
                    4 => f32::from_ne_bytes(buf[..].try_into().unwrap()) as f64,
                    8 => f64::from_ne_bytes(buf[..].try_into().unwrap()),
                    _ => unreachable!(),
                },
                |new| match N {
                    4 => {
                        if let Ok(val) = new.parse::<f32>() {
                            ctx.process.write(address, &val.to_ne_bytes());
                            true
                        } else {
                            false
                        }
                    }
                    8 => {
                        if let Ok(val) = new.parse::<f64>() {
                            ctx.process.write(address, &val.to_ne_bytes());
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
            );
        });

        ctx.offset += N;
        None
    }

    fn codegen(&self, generator: &mut dyn Generator, _: &CodegenData) {
        generator.add_field(
            self.state.name.borrow().as_str(),
            match N {
                4 => FieldKind::F32,
                8 => FieldKind::F64,
                _ => unreachable!(),
            },
            None,
        );
    }

    fn name(&self) -> Option<String> {
        Some(self.state.name.borrow().clone())
    }
}
