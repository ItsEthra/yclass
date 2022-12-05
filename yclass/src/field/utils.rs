use super::{create_text_format, EditingState, Field, HexField, NamedState};
use crate::{app::is_valid_ident, context::InspectionContext, FID_M};
use eframe::{
    egui::{FontSelection, Label, Sense, TextEdit, Ui},
    epaint::{text::LayoutJob, Color32, Stroke},
};

pub fn display_field_prelude(field: &dyn Field, ctx: &mut InspectionContext, job: &mut LayoutJob) {
    job.append(&format!("{:04X}", ctx.offset), 0., {
        let mut tf = create_text_format(
            ctx.is_selected(field.id()),
            Color32::KHAKI,
            // Highlight unaligned fields
        );
        if ctx.offset % 8 != 0 {
            tf.underline = Stroke::new(1., Color32::RED);
        }

        tf
    });
    job.append(
        &format!("{:012X}", ctx.address + ctx.offset),
        8.,
        create_text_format(ctx.is_selected(field.id()), Color32::LIGHT_GREEN),
    );
}

pub fn display_field_value(
    field: &dyn Field,
    ui: &mut Ui,
    ctx: &mut InspectionContext,
    state: &NamedState,
    displayed_value: impl FnOnce() -> String,
    write_new_value: impl FnOnce(&str) -> bool,
) {
    let editing_value = &mut *state.editing_state.borrow_mut();
    if let Some(EditingState {
        address,
        should_focus,
        buf,
    }) = editing_value
    {
        if *address == ctx.address + ctx.offset {
            let mut w = buf
                .chars()
                .map(|c| ui.fonts().glyph_width(&FID_M, c))
                .sum::<f32>();
            if w > 80. {
                w += 10.
            } else {
                w = 80.
            };

            let r = TextEdit::singleline(buf).desired_width(w).show(ui).response;
            if *should_focus {
                r.request_focus();
                *should_focus = false;
            }

            if r.clicked_elsewhere() {
                *editing_value = None;
            } else if r.lost_focus() {
                if !write_new_value(buf) {
                    ctx.toasts.error("Invalid value");
                    *should_focus = true;
                } else {
                    *editing_value = None;
                }
            }

            return;
        }
    }

    let mut job = LayoutJob::default();
    let displyed = displayed_value();
    job.append(
        &displyed,
        0.,
        create_text_format(ctx.is_selected(field.id()), Color32::WHITE),
    );

    let r = ui.add(Label::new(job).sense(Sense::click()));
    if r.secondary_clicked() {
        *editing_value = Some(EditingState::new(ctx.address + ctx.offset, displyed));
    }
}

pub fn display_field_name(
    field: &dyn Field,
    ui: &mut Ui,
    ctx: &mut InspectionContext,
    state: &NamedState,
    color: Color32,
) {
    if state
        .renaming_address
        .get()
        .map(|e_addr| e_addr == ctx.address + ctx.offset)
        .unwrap_or_default()
    {
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

        if state
            .focused_address
            .get()
            .map(|foc_addr| foc_addr == ctx.address + ctx.offset)
            .unwrap_or_default()
        {
            r.request_focus();
            state.focused_address.set(None);
        }

        if r.clicked_elsewhere() {
            *name = std::mem::take(&mut *state.saved_name.borrow_mut());
            state.renaming_address.set(None);
        } else if r.lost_focus() {
            if !is_valid_ident(name) {
                ctx.toasts.error("Not a valid field name");
                state.focused_address.set(Some(ctx.address + ctx.offset));
            } else {
                state.renaming_address.set(None);
            }
        }
    } else {
        let mut job = LayoutJob::default();
        job.append(
            state.name.borrow().as_ref(),
            0.,
            create_text_format(ctx.is_selected(field.id()), color),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.secondary_clicked() {
            *state.saved_name.borrow_mut() = state.name.borrow().clone();
            state.renaming_address.set(Some(ctx.address + ctx.offset));
            state.focused_address.set(Some(ctx.address + ctx.offset));
        } else if r.clicked() {
            ctx.select(field.id());
        }
    }
}

pub fn allocate_padding(mut n: usize) -> Vec<Box<dyn Field>> {
    let mut fields = vec![];

    while n >= 8 {
        fields.push(Box::new(HexField::<8>::new()) as _);
        n -= 8;
    }

    while n >= 4 {
        fields.push(Box::new(HexField::<4>::new()) as _);
        n -= 4;
    }

    while n >= 2 {
        fields.push(Box::new(HexField::<2>::new()) as _);
        n -= 2;
    }

    while n > 0 {
        fields.push(Box::new(HexField::<1>::new()) as _);
        n -= 1;
    }

    fields
}
