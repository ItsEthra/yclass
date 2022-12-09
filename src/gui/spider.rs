use super::{TextEditBind, TextEditFromStrBind};
use crate::{
    address::parse_address, field::FieldKind, process::Process, state::StateRef, value::Value,
};
use eframe::{
    egui::{ComboBox, Context, TextEdit, Ui, Window},
    epaint::FontId,
};
use egui_extras::{Column, TableBuilder};
use std::{iter::repeat, rc::Rc};

#[derive(PartialEq, Clone, Copy)]
enum FilterMode {
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Equal,
    NotEqual,
    Changed,
    Unchanged,
}

impl FilterMode {
    const NAMED_VARIANTS: &[(Self, &'static str)] = &[
        (Self::Greater, "Greater"),
        (Self::GreaterEq, "Greater or Equal"),
        (Self::Less, "Less"),
        (Self::LessEq, "Less or Equal"),
        (Self::Equal, "Equal"),
        (Self::NotEqual, "Not equal"),
        (Self::Changed, "Changed"),
        (Self::Unchanged, "Unchanged"),
    ];

    fn label(&self) -> &'static str {
        Self::NAMED_VARIANTS
            .iter()
            .find_map(|(v, s)| if v == self { Some(*s) } else { None })
            .unwrap()
    }
}

#[derive(Debug)]
struct SearchResult {
    // This should optimize memory usage for large amount of offsets,
    // We aren't modifying them anyways.
    parent_offsets: Rc<Vec<usize>>,
    offset: usize,
    last_value: Value,
}

impl SearchResult {
    pub fn should_remain(
        &mut self,
        p: &Process,
        mut address: usize,
        filter: FilterMode,
        new_value: Value,
    ) -> bool {
        let mut buf = [0; 8];

        for offset in self.parent_offsets.iter() {
            p.read(address + offset, &mut buf[..]);
            address = usize::from_ne_bytes(buf);
        }
        p.read(address + self.offset, &mut buf[..]);
        address = usize::from_ne_bytes(buf);

        p.read(address, &mut buf[..]);

        let current_value = bytes_to_value(&buf, self.last_value.kind());

        match filter {
            FilterMode::Less => current_value < new_value,
            FilterMode::LessEq => current_value <= new_value,
            FilterMode::Greater => current_value > new_value,
            FilterMode::GreaterEq => current_value >= new_value,
            FilterMode::Equal => current_value == new_value,
            FilterMode::NotEqual => current_value != new_value,
            FilterMode::Changed => current_value != self.last_value,
            FilterMode::Unchanged => current_value == self.last_value,
        }
    }
}

struct SearchOptions {
    offsets: Rc<Vec<usize>>,
    struct_size: usize,
    alignment: usize,
    address: usize,
    depth: usize,
    value: Value,
}

pub struct SpiderWindow {
    state: StateRef,
    shown: bool,

    max_levels: TextEditFromStrBind<usize>,
    struct_size: TextEditFromStrBind<usize>,
    alignment: TextEditFromStrBind<usize>,
    field_kind: FieldKind,

    base_address: TextEditBind<usize, ()>,
    value_buf: String,

    results: Vec<SearchResult>,
    filter: FilterMode,
}

impl SpiderWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            alignment: TextEditFromStrBind::new_from_str_with("4", Some(4)),
            max_levels: TextEditFromStrBind::new_from_str_with("2", Some(2)),
            struct_size: TextEditFromStrBind::new_from_str_with("256", Some(256)),
            field_kind: FieldKind::I32,

            base_address: TextEditBind::new(|s| parse_address(s).ok_or(())),
            value_buf: String::new(),

            filter: FilterMode::Equal,
            results: vec![],
            shown: false,
            state,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn show(&mut self, ctx: &Context) -> eyre::Result<Option<()>> {
        let shown = unsafe { &mut (*(self as *mut Self)).shown };

        Window::new("Structure spider")
            .open(shown)
            .show(ctx, |ui| {
                let state = &mut *self.state.borrow_mut();

                let Some(process) = state.process.as_ref() else {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Attach to a process first");
                    });

                    return Ok(());
                };

                fn show_edit<T, E>(
                    enabled: bool,
                    ui: &mut Ui,
                    bind: &mut TextEditBind<T, E>,
                    label: &str,
                ) {
                    let w = ui.available_width() / 2.;

                    ui.horizontal(|ui| {
                        ui.set_enabled(enabled);
                        ui.add(TextEdit::singleline(bind).desired_width(w));
                        ui.label(label);
                    });
                }

                let unlocked = self.results.is_empty();
                show_edit(unlocked, ui, &mut self.max_levels, "Max levels");
                show_edit(unlocked, ui, &mut self.struct_size, "Structure size");
                show_edit(unlocked, ui, &mut self.alignment, "Alignment");
                ui.scope(|ui| {
                    ui.set_enabled(unlocked);

                    ComboBox::new("_spider_select_kind", "Field type")
                        .width(ui.available_width() / 2. + 8. /* No clue */)
                        .selected_text(self.field_kind.label().unwrap())
                        .show_ui(ui, |ui| {
                            for (var, label) in FieldKind::NAMED_VARIANTS {
                                if ui
                                    .selectable_label(*var == self.field_kind, *label)
                                    .clicked()
                                {
                                    self.field_kind = *var;
                                }
                            }
                        });
                });

                ui.separator();

                let w = ui.available_width() / 2.;
                show_edit(true, ui, &mut self.base_address, "Base address");
                ui.horizontal(|ui| {
                    ui.add(TextEdit::singleline(&mut self.value_buf).desired_width(w));
                    ui.label("Value");
                });

                ui.separator();

                if self.results.is_empty() {
                    if ui.button("First search").clicked() {
                        let opts = self.collect_options()?;
                        recursive_first_search(process, &mut self.results, &opts);
                    }
                } else {
                    ComboBox::new("_spider_filter_box", "Filter")
                        .selected_text(self.filter.label())
                        .show_ui(ui, |ui| {
                            for (var, label) in FilterMode::NAMED_VARIANTS {
                                if ui.selectable_label(*var == self.filter, *label).clicked() {
                                    self.filter = *var;
                                }
                            }
                        });

                    let inner: eyre::Result<()> = ui
                        .horizontal(|ui| {
                            if ui.button("Next search").clicked() {
                                let address = self
                                    .base_address
                                    .value_clone()
                                    .map(|v| {
                                        v.map_err(|_| {
                                            eyre::eyre!("Base adderss is in invalid format")
                                        })
                                    })
                                    .ok_or(eyre::eyre!("Base address is required"))??;
                                let value = parse_kind_to_value(self.field_kind, &self.value_buf)?;

                                self.results.retain_mut(|r| {
                                    r.should_remain(process, address, self.filter, value)
                                });
                            }

                            if ui.button("Clear results").clicked() {
                                self.results.clear();
                            }

                            Ok(())
                        })
                        .inner;
                    _ = inner?;

                    ui.separator();

                    self.display_results(process, ui);
                }

                Ok(())
            })
            .map(|v| v.inner)
            .flatten()
            .transpose()
    }

    fn display_results(&mut self, process: &Process, ui: &mut Ui) {
        const DATA_HEIGHT: f32 = 14.;
        ui.style_mut().override_font_id = Some(FontId::monospace(DATA_HEIGHT));

        let Some(address) = self.base_address
            .value()
            .map(|v| v.ok())
            .flatten()
            .cloned()
        else {
            ui.heading("Invalid base address");
            return;
        };

        let levels = *self.max_levels.value().unwrap().unwrap();
        let w = ui.available_width() / (levels + 2) as f32;

        TableBuilder::new(ui)
            .striped(true)
            .columns(Column::initial(w).resizable(true), levels + 1)
            .column(Column::remainder())
            .header(16., |mut row| {
                for i in 1..=levels {
                    row.col(|ui| _ = ui.label(format!("{i}")));
                }

                row.col(|ui| _ = ui.label("Last"));
                row.col(|ui| _ = ui.label("Current"));
            })
            .body(|body| {
                body.rows(DATA_HEIGHT, self.results.len(), |idx, mut row| {
                    let result = &self.results[idx];

                    for offset in result.parent_offsets.iter() {
                        row.col(|ui| _ = ui.label(format!("{offset:X}")));
                    }

                    row.col(|ui| _ = ui.label(format!("{:X}", result.offset)));

                    // Without this, results with shorter offset path look weird.
                    for _ in repeat("").take(levels - result.parent_offsets.len() - 1) {
                        row.col(|ui| _ = ui.label(""));
                    }

                    row.col(|ui| _ = ui.label(format!("{}", result.last_value)));

                    let mut address = address;
                    let mut buf = [0; 8];
                    for offset in result.parent_offsets.iter() {
                        process.read(address + offset, &mut buf[..]);
                        address = usize::from_ne_bytes(buf);
                    }

                    process.read(address + result.offset, &mut buf[..]);
                    address = usize::from_ne_bytes(buf);

                    process.read(address, &mut buf[..]);

                    let current = bytes_to_value(&buf, result.last_value.kind());
                    row.col(|ui| _ = ui.label(format!("{}", current)));
                })
            })
    }

    fn collect_options(&self) -> eyre::Result<SearchOptions> {
        macro_rules! annotated {
            ($field:ident, $label:literal) => {
                self.$field
                    .value_clone()
                    .map(|v| v.map_err(|e| eyre::eyre!("{}: {e}", $label)))
                    .ok_or(eyre::eyre!(concat!($label, " is required")))??
            };
        }

        let depth = annotated!(max_levels, "Max level");
        let alignment = annotated!(alignment, "Alignment");
        let struct_size = annotated!(struct_size, "Struct size");
        let address = self
            .base_address
            .value_clone()
            .map(|v| v.map_err(|_| eyre::eyre!("Base adderss is in invalid format")))
            .ok_or(eyre::eyre!("Base address is required"))??;

        let value = parse_kind_to_value(self.field_kind, &self.value_buf)?;

        Ok(SearchOptions {
            offsets: Rc::default(),
            struct_size,
            alignment,
            address,
            depth,
            value,
        })
    }
}

fn recursive_first_search(
    process: &Process,
    results: &mut Vec<SearchResult>,
    opts: &SearchOptions,
) {
    if opts.depth == 0 {
        return;
    }

    let start = opts.address
        + if opts.address % opts.alignment == 0 {
            0
        } else {
            opts.alignment - opts.address % opts.alignment
        };

    for address in (start..start + opts.struct_size).step_by(opts.alignment) {
        let mut buf = [0; 8];
        process.read(address, &mut buf[..]);

        if address % 8 == 0 && process.can_read(usize::from_ne_bytes(buf)) {
            recursive_first_search(
                process,
                results,
                &SearchOptions {
                    offsets: Rc::new(
                        opts.offsets
                            .iter()
                            .copied()
                            .chain([address - start])
                            .collect(),
                    ),
                    address: usize::from_ne_bytes(buf),
                    struct_size: opts.struct_size,
                    alignment: opts.alignment,
                    depth: opts.depth - 1,
                    value: opts.value,
                },
            );
        }

        let value = bytes_to_value(&buf, opts.value.kind());

        if value == opts.value {
            results.push(SearchResult {
                parent_offsets: opts.offsets.clone(),
                offset: address - start,
                last_value: value,
            });
        }
    }
}

fn bytes_to_value(arr: &[u8; 8], kind: FieldKind) -> Value {
    macro_rules! into_value {
        ($s:ident, $type:ty) => {
            <$type>::from_ne_bytes(arr[..std::mem::size_of::<$type>()].try_into().unwrap()).into()
        };
    }

    match kind {
        FieldKind::I8 => into_value!(s, i8),
        FieldKind::I16 => into_value!(s, i16),
        FieldKind::I32 => into_value!(s, i32),
        FieldKind::I64 => into_value!(s, i64),
        FieldKind::U8 => into_value!(s, u8),
        FieldKind::U16 => into_value!(s, u16),
        FieldKind::U32 => into_value!(s, u32),
        FieldKind::U64 => into_value!(s, u64),
        FieldKind::F32 => into_value!(s, f32),
        FieldKind::F64 => into_value!(s, f64),
        _ => unreachable!(),
    }
}

fn parse_kind_to_value(kind: FieldKind, s: &str) -> eyre::Result<Value> {
    macro_rules! into_value {
        ($s:ident, $type:ty) => {
            $s.parse::<$type>()
                .map_err(|e| eyre::eyre!("Value: {e}"))?
                .into()
        };
    }

    Ok(match kind {
        FieldKind::I8 => into_value!(s, i8),
        FieldKind::I16 => into_value!(s, i16),
        FieldKind::I32 => into_value!(s, i32),
        FieldKind::I64 => into_value!(s, i64),
        FieldKind::U8 => into_value!(s, u8),
        FieldKind::U16 => into_value!(s, u16),
        FieldKind::U32 => into_value!(s, u32),
        FieldKind::U64 => into_value!(s, u64),
        FieldKind::F32 => into_value!(s, f32),
        FieldKind::F64 => into_value!(s, f64),
        _ => unreachable!(),
    })
}
