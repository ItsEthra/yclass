use super::{create_text_format, Field, FieldResponse, NamedState};
use crate::{context::InspectionContext, FID_M};
use eframe::{
    egui::{FontSelection, Label, Sense, TextEdit, Ui},
    epaint::{text::LayoutJob, Color32, Stroke},
};

pub fn display_field_prelude(field: &dyn Field, ctx: &mut InspectionContext, job: &mut LayoutJob) {
    let InspectionContext {
        address,
        selected,
        offset,
        ..
    } = ctx;

    job.append(&format!("{:04X}", *offset), 0., {
        let mut tf = create_text_format(
            *selected == Some(field.id()),
            Color32::KHAKI,
            // Highlight unaligned fields
        );
        if *offset % 8 != 0 {
            tf.underline = Stroke::new(1., Color32::RED);
        }

        tf
    });
    job.append(
        &format!("{:012X}", *address + *offset),
        8.,
        create_text_format(*selected == Some(field.id()), Color32::LIGHT_GREEN),
    );
}

pub fn display_field_name(
    field: &dyn Field,
    ui: &mut Ui,
    ctx: &InspectionContext,
    state: &NamedState,
    color: Color32,
) -> Option<FieldResponse> {
    let mut response = None;

    if state.editing.get() {
        let name = &mut *state.name.borrow_mut();
        let w = name
            .chars()
            .map(|c| ui.fonts().glyph_width(&FID_M, c))
            .sum::<f32>()
            .max(80.)
            + 32.;

        let r = TextEdit::singleline(name)
            .desired_width(w)
            .font(FontSelection::FontId(FID_M))
            .show(ui)
            .response;

        if state.request_focus.get() {
            r.request_focus();
            state.request_focus.set(false);
        }

        if !r.clicked_elsewhere() && r.lost_focus() {
            state.editing.set(false);
        }
    } else {
        let mut job = LayoutJob::default();
        job.append(
            state.name.borrow().as_ref(),
            0.,
            create_text_format(ctx.selected == Some(field.id()), color),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.double_clicked() {
            state.editing.set(true);
            state.request_focus.set(true);
        } else if r.clicked() {
            response = Some(FieldResponse::Selected(field.id()));
        }
    }

    response
}
