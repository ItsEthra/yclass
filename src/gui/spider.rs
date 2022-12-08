//! I feel like everything what's written in this file is very very scuffed

use crate::{
    address::parse_address, field::FieldKind, process::Process, state::StateRef, value::Value,
};
use eframe::egui::{ComboBox, Context, TextEdit, Ui, Window};
use std::rc::Rc;

#[derive(Debug)]
struct SearchResult {
    parent_offsets: Rc<Vec<usize>>,
    base_address: usize,
    last_value: Value,
    offset: usize,
}

impl SearchResult {
    fn should_remain(&self, proc: &Process, new_base_address: usize, new_value: Value) -> bool {
        true
    }
}

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
}

struct SearchOptions {
    base_address: usize,
    alignment: usize,
    value: Value,
    offsets: Rc<Vec<usize>>,
    level: usize,
    struct_size: usize,
}

pub struct SpiderWindow {
    state: StateRef,
    shown: bool,
    base_address: String,
    value: String,
    max_levels: String,
    alignment: String,
    struct_size: String,
    results: Vec<SearchResult>,
    kind: FieldKind,
    filter: FilterMode,
}

impl SpiderWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            shown: false,
            base_address: String::new(),
            value: String::new(),
            max_levels: String::new(),
            struct_size: String::new(),
            kind: FieldKind::I32,
            alignment: "4".to_owned(),
            results: vec![],
            filter: FilterMode::Equal,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    fn show_inital_values(&mut self, ui: &mut Ui) {
        let w = ui.available_width() / 2.;

        let named_text_edit = |ui: &mut Ui, lock: bool, label: &'static str, buf: &mut String| {
            ui.horizontal(|ui| {
                ui.set_enabled(self.results.is_empty() || !lock);

                TextEdit::singleline(buf)
                    .desired_width(w)
                    .hint_text(label)
                    .show(ui);
                ui.label(label);
            });
        };

        named_text_edit(ui, true, "Max levels", &mut self.max_levels);
        named_text_edit(ui, true, "Structure size", &mut self.struct_size);
        named_text_edit(ui, true, "Alignment", &mut self.alignment);
        ui.add_enabled_ui(self.results.is_empty(), |ui| {
            ComboBox::new("_spider_select_kind", "Type")
                .width(w)
                .selected_text(
                    FieldKind::NAMED_VARIANTS
                        .iter()
                        .find_map(|(k, s)| if *k == self.kind { Some(*s) } else { None })
                        .unwrap(),
                )
                .show_ui(ui, |ui| {
                    for (k, s) in FieldKind::NAMED_VARIANTS {
                        if ui.selectable_label(*k == self.kind, *s).clicked() {
                            self.kind = *k;
                            self.alignment = format!("{}", k.size());
                        }
                    }
                });
        });

        ui.separator();

        named_text_edit(ui, false, "Base address", &mut self.base_address);
        named_text_edit(ui, false, "Value", &mut self.value);
    }

    pub fn show(&mut self, ctx: &Context) -> eyre::Result<Option<()>> {
        // I promise not to use `self.shown` anywhere else.
        let shown = unsafe { &mut (*(self as *mut Self)).shown };

        Window::new("Structure spider")
            .open(shown)
            .show(ctx, |ui| {
                let state = self.state.borrow();
                let Some(process) = state.process.as_ref() else {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Select a process first");
                    });

                    return Ok(());
                };

                self.show_inital_values(ui);
                ui.separator();

                if self.results.is_empty() {
                    let inner: eyre::Result<()> = ui
                        .horizontal(|ui| {
                            if ui.button("Initial search").clicked() {
                                let opts = SearchOptions {
                                    offsets: Rc::default(),
                                    base_address: parse_address(&self.base_address)
                                        .ok_or(eyre::eyre!("Address is in invalid format"))?,
                                    alignment: self.alignment.parse::<usize>().map_err(|_| {
                                        eyre::eyre!("Alignment value is in invalid format")
                                    })?,
                                    value: parse_kind_to_value(self.kind, &self.value)
                                        .map_err(|_| eyre::eyre!("Value is in invalid format"))?,
                                    level: self.max_levels.parse::<usize>().map_err(|_| {
                                        eyre::eyre!("Max levels value is in invalid format")
                                    })?,
                                    struct_size: self.struct_size.parse::<usize>().map_err(
                                        |_| {
                                            eyre::eyre!("Structure size value is in invalid format")
                                        },
                                    )?,
                                };

                                recursive_first_search(
                                    process,
                                    &mut self.results,
                                    opts.base_address,
                                    &opts,
                                );
                            }

                            Ok(())
                        })
                        .inner;

                    _ = inner?;
                }

                if !self.results.is_empty() {
                    ComboBox::new("_spider_filter_box", "Filter")
                        .selected_text(
                            FilterMode::NAMED_VARIANTS
                                .iter()
                                .find_map(|(v, s)| if *v == self.filter { Some(*s) } else { None })
                                .unwrap(),
                        )
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
                                let new_base_address = parse_address(&self.base_address)
                                    .ok_or(eyre::eyre!("Address is in invalid format"))?;
                                let new_value = parse_kind_to_value(self.kind, &self.value)
                                    .map_err(|_| eyre::eyre!("Value is in invalid format"))?;

                                self.results.retain_mut(|sr| {
                                    sr.should_remain(process, new_base_address, new_value)
                                });
                            }

                            if ui.button("Clear").clicked() {
                                self.results.clear();
                            }

                            Ok(())
                        })
                        .inner;

                    _ = inner?;
                }

                ui.separator();

                Ok(())
            })
            .map(|i| i.inner)
            .flatten()
            .transpose()
    }
}

fn recursive_first_search(
    process: &Process,
    results: &mut Vec<SearchResult>,
    address: usize,
    opts: &SearchOptions,
) {
    if opts.level == 0 {
        return;
    }

    let slot = address
        + if address % opts.alignment != 0 {
            opts.alignment - address % opts.alignment
        } else {
            0
        };

    for addr in (slot..(slot + opts.struct_size)).step_by(opts.alignment) {
        let mut data = [0; 8];
        process.read(addr, &mut data[..]);

        if addr % 8 == 0 {
            let ptr = usize::from_ne_bytes(data);
            if process.can_read(ptr) {
                let mut new_offsets = (*opts.offsets).clone();
                new_offsets.push(addr - slot);

                recursive_first_search(
                    process,
                    results,
                    ptr,
                    &SearchOptions {
                        base_address: opts.base_address,
                        alignment: opts.alignment,
                        value: opts.value,
                        level: opts.level - 1,
                        struct_size: opts.struct_size,
                        offsets: Rc::new(new_offsets),
                    },
                );
            }
        }

        let value = bytes_to_value(&data, opts.value.kind());
        if value == opts.value {
            results.push(SearchResult {
                parent_offsets: opts.offsets.clone(),
                base_address: opts.base_address,
                offset: addr - slot,
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
            $s.parse::<$type>()?.into()
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
