use super::{
    create_text_format, display_field_name, display_field_prelude, next_id, CodegenData, Field,
    FieldId, FieldKind, FieldResponse, NamedState,
};
use crate::{context::InspectionContext, generator::Generator, FID_M};
use eframe::{
    egui::{
        collapsing_header::CollapsingState, popup_below_widget, Id, Key, Label, RichText, Sense, Ui,
    },
    epaint::{text::LayoutJob, Color32},
};
use std::cell::Cell;

pub struct PointerField {
    id: FieldId,
    state: NamedState,
    class_id: Cell<Option<usize>>,
}

impl PointerField {
    pub fn new(name: String) -> Self {
        Self {
            id: next_id(),
            state: NamedState::new(name),
            class_id: None.into(),
        }
    }

    fn show_header(&self, ui: &mut Ui, ctx: &mut InspectionContext, address: usize) {
        let class = self
            .class_id
            .get()
            .map(|id| ctx.class_list.by_id(id))
            .flatten();

        let (text, exists) = if let Some(cl) = class {
            (format!("[{}]", cl.name), true)
        } else {
            (format!("[C{:X}]", address), false)
        };

        let mut job = LayoutJob::default();
        display_field_prelude(self, ctx, &mut job);
        if ui.add(Label::new(job).sense(Sense::click())).clicked() {
            ctx.select(self.id);
        }

        display_field_name(self, ui, ctx, &self.state, Color32::BROWN);

        let is_selected = ctx.is_selected(self.id);
        let uniq_id = ctx.address + ctx.offset + self.id as usize;

        let mut job = LayoutJob::default();
        job.append(
            &format!(" -> {address:X}"),
            0.,
            create_text_format(is_selected, Color32::YELLOW),
        );
        if ui.add(Label::new(job).sense(Sense::click())).clicked() {
            ctx.select(self.id);
        }

        let mut job = LayoutJob::default();
        job.append(
            &text,
            4.,
            create_text_format(
                is_selected,
                if exists {
                    Color32::LIGHT_GRAY
                } else {
                    Color32::DARK_GRAY
                },
            ),
        );

        let r = ui.add(Label::new(job).sense(Sense::click()));
        if r.secondary_clicked() {
            ui.memory().toggle_popup(Id::new(uniq_id))
        } else if r.clicked() {
            ctx.select(self.id);
        }

        popup_below_widget(ui, Id::new(uniq_id), &r, |ui| {
            ui.set_width(80.);
            ui.vertical_centered_justified(|ui| {
                for cl in ctx.class_list.classes() {
                    if ui.button(&cl.name).clicked() {
                        self.class_id.set(Some(cl.id()));
                    }
                }
            });
        });
    }

    fn show_body(
        &self,
        ui: &mut Ui,
        ctx: &mut InspectionContext,
        address: usize,
    ) -> Option<FieldResponse> {
        if !ctx.process.can_read(address) {
            ui.heading(
                RichText::new("Pointer's body is only allowed at valid addresses")
                    .color(Color32::RED)
                    .font(FID_M),
            );
            return None;
        }

        let mut response = None;

        let cid = self.class_id.get()?;
        if let Some(class) = ctx.class_list.by_id(cid) {
            let mut inner_ctx = InspectionContext {
                class_list: ctx.class_list,
                selection: ctx.selection,
                current_container: cid,
                process: ctx.process,
                toasts: ctx.toasts,
                offset: 0,
                address,
            };

            match class
                .fields
                .iter()
                .fold(None, |r, f| r.or(f.draw(ui, &mut inner_ctx)))
            {
                Some(other) => response = Some(other),
                None => {}
            }
            ctx.selection = inner_ctx.selection;
        } else {
            response = Some(FieldResponse::NewClass(format!("C{:X}", address), cid));
        }

        response
    }
}

impl Field for PointerField {
    fn id(&self) -> FieldId {
        self.id
    }

    fn size(&self) -> usize {
        // TODO(ItsEthra): When inspecting 32-bit processes
        // size of the pointer would be `4`. But I am not sure
        // if the rest of this app isn't break in this case lol.
        8
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut response = None;

        // TODO(ItsEthra): Again, pointer size differs in 32-bit processes.
        let mut buf = [0; 8];
        ctx.process.read(ctx.address + ctx.offset, &mut buf);
        let address = usize::from_ne_bytes(buf);

        let uniq_id = ctx.address + ctx.offset + self.id as usize;
        if self.class_id.get().is_none() {
            self.class_id.set(Some(fastrand::usize(..)));
        }

        let mut state = CollapsingState::load_with_default_open(ui.ctx(), Id::new(uniq_id), false);
        if ctx.is_selected(self.id)
            && ui.input().key_pressed(Key::Space)
            && !self.state.editing_address.get().is_some()
        // TODO(ItsEthra): questionable line of code
        {
            state.toggle(ui);
        }

        let body = state
            .show_header(ui, |ui| self.show_header(ui, ctx, address))
            .body(|ui| self.show_body(ui, ctx, address))
            .2;
        let body = body.map(|inner| inner.inner).flatten();

        if let Some(new) = body {
            response = Some(new);
        }

        ctx.offset += self.size();
        response
    }

    fn codegen(&self, generator: &mut dyn Generator, data: &CodegenData) {
        generator.add_field(
            self.state.name.borrow().as_str(),
            FieldKind::Ptr,
            Some(
                &data
                    .classes
                    .iter()
                    .find(|c| c.id() == self.class_id.get().unwrap())
                    .unwrap()
                    .name,
            ),
        );
    }

    fn name(&self) -> Option<String> {
        Some(self.state.name.borrow().clone())
    }
}
