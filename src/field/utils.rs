use super::{create_text_format, EditingState, Field, HexField, NamedState};
use crate::{app::is_valid_ident, context::InspectionContext, FID_M};
use eframe::{
    egui::{Context, FontSelection, Key, Label, Modifiers, Sense, TextEdit, Ui},
    epaint::{text::LayoutJob, Color32, Stroke},
};
use std::fmt::Display;

pub fn display_field_prelude(
    egui_ctx: &Context,
    field: &dyn Field,
    ctx: &mut InspectionContext,
    job: &mut LayoutJob,
) {
    job.append(&format!("{:04X}", ctx.offset), 0., {
        let mut tf = create_text_format(ctx.is_selected(field.id()), Color32::KHAKI);
        // Highlight unaligned fields
        if ctx.offset % 8 != 0 {
            tf.underline = Stroke::new(1., Color32::RED);
        }

        if egui_ctx.input().key_pressed(Key::C)
            && egui_ctx.input().modifiers.matches(Modifiers::CTRL)
            && ctx.is_selected(field.id())
        {
            egui_ctx.output().copied_text = format!("{:X}", ctx.address + ctx.offset);
        }

        if egui_ctx.input().key_pressed(Key::C)
            && egui_ctx
                .input()
                .modifiers
                .matches(Modifiers::CTRL | Modifiers::SHIFT)
            && ctx.is_selected(field.id())
        {
            let mut buf = [0; 8];
            ctx.process.read(ctx.address + ctx.offset, &mut buf[..]);
            egui_ctx.output().copied_text = format!("{:X}", usize::from_ne_bytes(buf));
        }

        tf
    });
    job.append(
        &format!("{:012X}", ctx.address + ctx.offset),
        8.,
        create_text_format(ctx.is_selected(field.id()), Color32::LIGHT_GREEN),
    );
}

pub fn display_field_value<T: Display>(
    field: &dyn Field,
    ui: &mut Ui,
    ctx: &mut InspectionContext,
    state: &NamedState,
    color: Color32,
    // if `bool` is `true` it indicates that
    // the value returned would be used as initial value for
    // text edit box.
    mut displayed_value: impl FnMut(bool) -> T,
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
    job.append(
        &displayed_value(false).to_string(),
        0.,
        create_text_format(ctx.is_selected(field.id()), color),
    );

    let r = ui.add(Label::new(job).sense(Sense::click()));
    if r.secondary_clicked() {
        *editing_value = Some(EditingState::new(
            ctx.address + ctx.offset,
            displayed_value(true).to_string(),
        ));
    } else if r.clicked() {
        ctx.select(field.id());
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
        .renaming_id
        .get()
        .map(|uid| uid == ctx.current_id)
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
            .focused_id
            .get()
            .map(|uid| uid == ctx.current_id)
            .unwrap_or_default()
        {
            r.request_focus();
            state.focused_id.set(None);
        }

        if r.clicked_elsewhere() {
            *name = std::mem::take(&mut *state.saved_name.borrow_mut());
            state.renaming_id.set(None);
        } else if r.lost_focus() {
            if !is_valid_ident(name) {
                ctx.toasts.error("Not a valid field name");
                state.focused_id.set(Some(ctx.current_id));
            } else {
                state.renaming_id.set(None);
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
            state.renaming_id.set(Some(ctx.current_id));
            state.focused_id.set(Some(ctx.current_id));
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
