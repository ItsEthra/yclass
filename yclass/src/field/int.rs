use super::{
    create_text_format, display_field_name, display_field_prelude, next_id, Field, FieldId,
    FieldResponse, NamedState,
};
use crate::context::InspectionContext;
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

    fn show_value(&self, ui: &mut Ui, ctx: &InspectionContext) -> Option<FieldResponse> {
        let mut response = None;

        let mut buf = [0; N];
        ctx.process.read(ctx.address + ctx.offset, &mut buf[..]);
        let displayed = match N {
            1 => buf[0] as i8 as i64,
            2 => i16::from_ne_bytes(buf[..].try_into().unwrap()) as i64,
            4 => i32::from_ne_bytes(buf[..].try_into().unwrap()) as i64,
            8 => i64::from_ne_bytes(buf[..].try_into().unwrap()),
            _ => unreachable!(),
        };

        let mut job = LayoutJob::default();
        job.append(
            &format!("{displayed}"),
            0.,
            create_text_format(ctx.selected == Some(self.id), Color32::WHITE),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            response = Some(FieldResponse::Selected(self.id));
        }

        response
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
        let mut response = None;

        let mut buf = [0; N];
        ctx.process.read(ctx.address + ctx.offset, &mut buf);

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                response = Some(FieldResponse::Selected(self.id));
            }

            if let Some(new) = display_field_name(
                self,
                ui,
                ctx,
                &self.state,
                if self.signed {
                    Color32::LIGHT_BLUE
                } else {
                    Color32::LIGHT_GREEN
                },
            ) {
                response = Some(new);
            }

            if let Some(new) = self.show_value(ui, ctx) {
                response = Some(new);
            }
        });

        ctx.offset += N;
        response
    }
}
