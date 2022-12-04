use super::{
    create_text_format, display_field_name, display_field_prelude, next_id, Field, FieldId,
    FieldResponse, NamedState,
};
use crate::context::InspectionContext;
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

    fn show_value(&self, ui: &mut Ui, ctx: &InspectionContext) -> Option<FieldResponse> {
        let mut response = None;

        let mut buf = [0; N];
        ctx.process.read(ctx.address + ctx.offset, &mut buf[..]);
        let displayed = match N {
            4 => f32::from_ne_bytes(buf[..].try_into().unwrap()) as f64,
            8 => f64::from_ne_bytes(buf[..].try_into().unwrap()),
            _ => unreachable!(),
        };

        let mut job = LayoutJob::default();
        job.append(
            &format!("{displayed}"),
            0.,
            create_text_format(ctx.selected == Some(self.id), Color32::WHITE),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()).wrap(true));
        if r.clicked() {
            response = Some(FieldResponse::Selected(self.id));
        }

        response
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
        let mut response = None;

        let mut buf = [0; N];
        ctx.process.read(ctx.address + ctx.offset, &mut buf);

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                response = Some(FieldResponse::Selected(self.id));
            }

            if let Some(new) = display_field_name(self, ui, ctx, &self.state, Color32::LIGHT_RED) {
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
