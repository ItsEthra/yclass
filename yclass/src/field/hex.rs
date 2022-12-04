use super::{create_text_format, display_field_prelude, next_id, Field, FieldId, FieldResponse};
use crate::context::InspectionContext;
use eframe::{
    egui::{Label, Sense, Ui},
    epaint::{text::LayoutJob, Color32},
};
use std::ops::RangeFrom;

pub struct HexField<const N: usize> {
    id: FieldId,
}

impl<const N: usize> HexField<N> {
    pub fn new() -> Self {
        Self { id: next_id() }
    }

    fn byte_view(&self, ctx: &mut InspectionContext, job: &mut LayoutJob, buf: &[u8; N]) {
        for (i, b) in buf.iter().enumerate() {
            let rng = fastrand::Rng::with_seed(*b as _);
            let color = if *b == 0 {
                Color32::DARK_GRAY
            } else {
                const MIN: RangeFrom<u8> = 45..;
                Color32::from_rgb(rng.u8(MIN), rng.u8(MIN), rng.u8(MIN))
            };

            job.append(
                &format!("{b:02X}"),
                4. + if i == 0 { 4. } else { 0. },
                create_text_format(ctx.is_selected(self.id), color),
            );
        }
    }

    fn int_view(&self, ui: &mut Ui, ctx: &mut InspectionContext, buf: &[u8; N]) {
        let mut job = LayoutJob::default();
        let (mut high, mut low) = (0i64, 0i64);

        let displayed = if N == 1 {
            buf[0] as i8 as i64
        } else {
            let half = N / 2;

            (high, low) = int_high_low_from_le::<N>(&buf[..half], &buf[half..]);

            match N {
                2 => i16::from_le_bytes(buf[..].try_into().unwrap()) as i64,
                4 => i32::from_le_bytes(buf[..].try_into().unwrap()) as i64,
                8 => i64::from_le_bytes(buf[..].try_into().unwrap()),
                _ => unreachable!(),
            }
        };

        job.append(
            &format!("{}", displayed),
            4.,
            create_text_format(ctx.is_selected(self.id), Color32::LIGHT_BLUE),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            ctx.select(self.id);
        }

        if N != 1 {
            r.on_hover_text(format!("High: {high}\nLow: {low}"));
        }
    }

    fn float_view(&self, ui: &mut Ui, ctx: &mut InspectionContext, buf: &[u8; N]) {
        if N != 4 && N != 8 {
            return;
        }

        let mut job = LayoutJob::default();

        let displayed = if N == 4 {
            f32::from_ne_bytes(buf[..].try_into().unwrap()) as f64
        } else {
            f64::from_ne_bytes(buf[..].try_into().unwrap())
        };

        job.append(
            &format!("{:e}", displayed),
            4.,
            create_text_format(ctx.is_selected(self.id), Color32::LIGHT_RED),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.clicked() {
            ctx.select(self.id);
        }

        if N == 8 {
            let (high, low) = (
                f32::from_ne_bytes(buf[..4].try_into().unwrap()),
                f32::from_ne_bytes(buf[4..].try_into().unwrap()),
            );

            r.on_hover_text(format!("Full:{displayed}\nHigh: {high}\nLow: {low}"));
        } else if N == 4 {
            r.on_hover_text(format!("Full:{displayed}"));
        }
    }
}

impl<const N: usize> Field for HexField<N> {
    fn id(&self) -> FieldId {
        self.id
    }

    fn size(&self) -> usize {
        N
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut buf = [0; N];
        ctx.process.read(ctx.address + ctx.offset, &mut buf);

        ui.horizontal(|ui| {
            let mut job = LayoutJob::default();
            display_field_prelude(self, ctx, &mut job);
            self.byte_view(ctx, &mut job, &buf);

            if ui.add(Label::new(job).sense(Sense::click())).clicked() {
                ctx.select(self.id);
            }

            self.int_view(ui, ctx, &buf);
            self.float_view(ui, ctx, &buf);
        });

        ctx.offset += N;
        None
    }
}

fn int_high_low_from_le<const N: usize>(high: &[u8], low: &[u8]) -> (i64, i64) {
    match N {
        8 => (
            i32::from_ne_bytes(high.try_into().unwrap()) as _,
            i32::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        4 => (
            i16::from_ne_bytes(high.try_into().unwrap()) as _,
            i16::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        2 => (
            i8::from_ne_bytes(high.try_into().unwrap()) as _,
            i8::from_ne_bytes(low.try_into().unwrap()) as _,
        ),
        _ => unreachable!(),
    }
}
