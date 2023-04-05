use super::{
    create_text_format, display_field_name, display_field_prelude, display_field_value, next_id,
    CodegenData, Field, FieldId, FieldKind, FieldResponse, NamedState,
};
use crate::{address::parse_address, context::InspectionContext, generator::Generator, FID_M};
use eframe::{
    egui::{
        collapsing_header::CollapsingState, popup_below_widget, Id, Label, RichText, Sense,
        TextFormat, Ui,
    },
    epaint::{text::LayoutJob, Color32},
};
use fastrand::Rng;
use std::{cell::Cell, mem::transmute};

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

    pub fn new_with_class_id(name: String, class_id: usize) -> Self {
        Self {
            id: next_id(),
            state: NamedState::new(name),
            class_id: Some(class_id).into(),
        }
    }

    fn show_header(&self, ui: &mut Ui, ctx: &mut InspectionContext, address: usize) {
        let class = self.class_id.get().and_then(|id| ctx.class_list.by_id(id));

        let (text, exists) = if let Some(cl) = class {
            (format!("[{}]", cl.name), true)
        } else {
            (format!("[C{:X}]", address), false)
        };

        let mut job = LayoutJob::default();
        display_field_prelude(ui.ctx(), self, ctx, &mut job);
        job.append(" ", 0., TextFormat::default());

        if ui.add(Label::new(job).sense(Sense::click())).clicked() {
            ctx.select(self.id);
        }

        display_field_name(self, ui, ctx, &self.state, Color32::BROWN);

        let is_selected = ctx.is_selected(self.id);
        let paddr = ctx.address + ctx.offset;

        ui.add_space(4.);

        display_field_value(
            self,
            ui,
            ctx,
            &self.state,
            Color32::YELLOW,
            |v| {
                if v {
                    format!("{address:X}")
                } else {
                    format!("-> {address:X}")
                }
            },
            |new| {
                if let Some(addr) = parse_address(new) {
                    ctx.process.write(paddr, &addr.to_ne_bytes());
                    true
                } else {
                    false
                }
            },
        );

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
            ui.memory_mut(|m| m.toggle_popup(Id::new(ctx.current_id)));
        } else if r.clicked() {
            ctx.select(self.id);
        }

        popup_below_widget(ui, Id::new(ctx.current_id), &r, |ui| {
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
                RichText::new(format!("Can't read memory at address {address:#X}"))
                    .color(Color32::RED)
                    .font(FID_M),
            );
            return None;
        }

        let mut response = None;

        let cid = self.class_id.get()?;
        if let Some(class) = ctx.class_list.by_id(cid) {
            let rng = Rng::with_seed(unsafe { transmute(ctx.current_id) });

            let mut inner_ctx = InspectionContext {
                class_list: ctx.class_list,
                parent_id: ctx.current_id,
                selection: ctx.selection,
                current_container: cid,
                // Will be immideately reassigned.
                current_id: Id::null(),
                process: ctx.process,
                toasts: ctx.toasts,
                level_rng: &rng,
                offset: 0,
                address,
            };

            #[allow(clippy::single_match)]
            match class.fields.iter().fold(None, |r, f| {
                inner_ctx.current_id = Id::new(rng.u64(..));
                r.or(f.draw(ui, &mut inner_ctx))
            }) {
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

    fn name(&self) -> Option<String> {
        Some(self.state.name.borrow().clone())
    }

    fn kind(&self) -> FieldKind {
        FieldKind::Ptr
    }

    fn draw(&self, ui: &mut Ui, ctx: &mut InspectionContext) -> Option<FieldResponse> {
        let mut response = None;

        // TODO(ItsEthra): Again, pointer size differs in 32-bit processes.
        let mut buf = [0; 8];
        ctx.process.read(ctx.address + ctx.offset, &mut buf);
        let address = usize::from_ne_bytes(buf);

        if self.class_id.get().is_none() {
            self.class_id.set(Some(fastrand::usize(..)));
        }

        let state = CollapsingState::load_with_default_open(ui.ctx(), ctx.current_id, false);
        let body = state
            .show_header(ui, |ui| self.show_header(ui, ctx, address))
            .body(|ui| self.show_body(ui, ctx, address))
            .2;
        let body = body.and_then(|inner| inner.inner);

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
            data.classes
                .iter()
                .find(|c| c.id() == self.class_id.get().unwrap())
                .map(|c| c.name.as_ref()),
        );
    }
}
