use super::{
    display_field_name, display_field_prelude, display_field_value, next_id, CodegenData, Field,
    FieldId, FieldKind, FieldResponse, NamedState,
};
use crate::{context::InspectionContext, generator::Generator, process::Process};
use eframe::{
    egui::{Label, Sense, Ui},
    epaint::{text::LayoutJob, Color32},
};

pub struct IntField<const N: usize> {
    id: FieldId,
    signed: bool,
    state: NamedState,
}

impl<const N: usize> IntField<N> {
    pub fn signed(name: String) -> Self {
        Self {
            id: next_id(),
            signed: true,
            state: NamedState::new(name),
        }
    }

    pub fn unsigned(name: String) -> Self {
        Self {
            id: next_id(),
            signed: false,
            state: NamedState::new(name),
        }
    }

    fn write_value(&self, new: &str, address: usize, proc: &Process) -> bool {
        macro_rules! do_arm {
            ($buf:ident, $addr:ident, $proc:ident, $new:ident, $type:ty) => {
                if let Ok(val) = $new.parse::<$type>() {
                    $proc.write($addr, &val.to_ne_bytes());
                    true
                } else {
                    false
                }
            };
        }

        match N {
            1 if self.signed => do_arm!(buf, address, proc, new, i8),
            1 if !self.signed => do_arm!(buf, address, proc, new, u8),
            2 if self.signed => do_arm!(buf, address, proc, new, i16),
            2 if !self.signed => do_arm!(buf, address, proc, new, u16),
            4 if self.signed => do_arm!(buf, address, proc, new, i32),
            4 if !self.signed => do_arm!(buf, address, proc, new, u32),
            8 if self.signed => do_arm!(buf, address, proc, new, i64),
            8 if !self.signed => do_arm!(buf, address, proc, new, u64),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize> Field for IntField<N> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn size(&self) -> usize {
        N
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut buf = [0; N];
        let address = ctx.address + ctx.offset;
        ctx.process.read(ctx.address + ctx.offset, &mut buf);

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                ctx.select(self.id);
            }

            display_field_name(
                self,
                ui,
                ctx,
                &self.state,
                if self.signed {
                    Color32::LIGHT_BLUE
                } else {
                    Color32::LIGHT_GREEN
                },
            );
            display_field_value(
                self,
                ui,
                ctx,
                &self.state,
                Color32::WHITE,
                |_| match N {
                    1 if self.signed => (buf[0] as i8).to_string(),
                    1 if !self.signed => (buf[0] as u8).to_string(),
                    2 if self.signed => i16::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    2 if !self.signed => u16::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    4 if self.signed => i32::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    4 if !self.signed => u32::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    8 if self.signed => i64::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    8 if !self.signed => u64::from_ne_bytes(buf[..].try_into().unwrap()).to_string(),
                    _ => unreachable!(),
                },
                |new| self.write_value(new, address, ctx.process),
            );
        });

        ctx.offset += N;
        None
    }

    fn codegen(&self, generator: &mut dyn Generator, _: &CodegenData) {
        generator.add_field(
            self.state.name.borrow().as_str(),
            match N {
                1 if self.signed => FieldKind::I8,
                1 if !self.signed => FieldKind::U8,
                2 if self.signed => FieldKind::I16,
                2 if !self.signed => FieldKind::U16,
                4 if self.signed => FieldKind::I32,
                4 if !self.signed => FieldKind::U32,
                8 if self.signed => FieldKind::I64,
                8 if !self.signed => FieldKind::U64,
                _ => unreachable!(),
            },
            None,
        );
    }

    fn name(&self) -> Option<String> {
        Some(self.state.name.borrow().clone())
    }
}
