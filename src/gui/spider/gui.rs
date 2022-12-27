use super::{FilterMode, ScannerReport, ScannerState, SearchResult};
use crate::{
    address::parse_address,
    field::FieldKind,
    gui::{
        spider::{bytes_to_value, parse_kind_to_value, SearchOptions},
        TextEditBind, TextEditFromStrBind,
    },
    process::Process,
    state::StateRef,
    value::Value,
};
use eframe::{
    egui::{Button, ComboBox, Context, RichText, TextEdit, Ui, Window},
    epaint::{vec2, Color32, FontId},
};
use egui_extras::{Column, TableBuilder};
use std::{borrow::Cow, iter::repeat, sync::Arc, time::Instant};

enum DisplayMode {
    Normal,
    Hex,
}

impl DisplayMode {
    fn format(&self, value: Value) -> String {
        macro_rules! match_val {
            ($value:ident, $fmt:literal, $($id:ident),*) => {
                match $value {
                    $(
                        Value::$id(v) => format!($fmt, v),
                    )*
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }

        match self {
            Self::Normal => match_val!(value, "{}", U8, I8, U16, I16, U32, I32, U64, I64, F32, F64),
            Self::Hex => match_val!(value, "{:X}", U8, I8, U16, I16, U32, I32, U64, I64),
        }
    }
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

    scanner_status: Option<Cow<'static, str>>,
    results: Vec<SearchResult>,
    filter: FilterMode,
    display: DisplayMode,

    scanner: ScannerState,
}

impl SpiderWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            alignment: TextEditFromStrBind::new_from_str_with("4", Some(4)),
            max_levels: TextEditFromStrBind::new_from_str_with("2", Some(2)),
            struct_size: TextEditFromStrBind::new_from_str_with("256", Some(256)),
            field_kind: FieldKind::I32,

            base_address: TextEditBind::new(|s| parse_address(s).ok_or(())),
            scanner: ScannerState::new(),

            display: DisplayMode::Normal,
            filter: FilterMode::Equal,
            value_buf: String::new(),
            scanner_status: None,
            results: vec![],
            shown: false,
            state,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;

        if self.shown && self.base_address.value().is_none() {
            let address = self
                .state
                .borrow()
                .selection
                .map(|s| s.address)
                .unwrap_or_else(|| self.state.borrow().inspect_address);
            self.base_address.set(address, format!("{address:X}"));
        }
    }

    pub fn show(&mut self, ctx: &Context) -> eyre::Result<Option<()>> {
        // I promise not to use self.show anywhere else.
        let shown = unsafe { &mut (*(self as *mut Self)).shown };

        match self.scanner.try_take() {
            ScannerReport::Finshed(time, mut results) => {
                results.sort_unstable_by_key(|v| v.parent_offsets.len());

                self.results = results;
                self.scanner_status =
                    Some(format!("Finished in: {:.2}", time.as_secs_f32()).into());
            }
            ScannerReport::InProgress => {
                self.scanner_status = Some("In progress".into());
            }
            ScannerReport::Idle => {}
        }

        Window::new("Structure spider")
            .open(shown)
            .show(ctx, |ui| {
                let state = &mut *self.state.borrow_mut();

                let process_lock = state.process.read();
                let Some(process) = process_lock.as_ref() else {
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

                let enabled = !self.scanner.active() && self.results.is_empty();
                show_edit(enabled, ui, &mut self.max_levels, "Max levels");
                show_edit(enabled, ui, &mut self.struct_size, "Structure size");
                show_edit(enabled, ui, &mut self.alignment, "Alignment");
                ui.scope(|ui| {
                    ui.set_enabled(enabled);

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
                                    self.alignment.set(var.size(), var.size().to_string());
                                }
                            }
                        });
                });

                ui.separator();

                let w = ui.available_width() / 2.;
                show_edit(true, ui, &mut self.base_address, "Base address");
                ui.horizontal(|ui| {
                    ui.add(TextEdit::singleline(&mut self.value_buf).desired_width(w));
                    if self.results.is_empty() {
                        ui.label("Initial value");
                    } else {
                        ui.label("Filter value");
                    }
                });

                ui.separator();

                if !self.scanner.active() && self.results.is_empty() {
                    if ui
                        .add_sized(vec2(w + 8., 12.), Button::new("First search"))
                        .clicked()
                    {
                        let opts = self.collect_options()?;
                        self.scanner.begin(&state.process, opts);
                    }
                } else {
                    ui.horizontal(|ui| {
                        ComboBox::new("_spider_filter_box", "Filter")
                            .selected_text(self.filter.label())
                            .show_ui(ui, |ui| {
                                for (var, label) in FilterMode::NAMED_VARIANTS {
                                    if ui.selectable_label(*var == self.filter, *label).clicked() {
                                        self.filter = *var;
                                    }
                                }
                            });

                        if let Some(status) = self.scanner_status.as_deref() {
                            ui.separator();
                            ui.label(status);
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

                                let time = Instant::now();
                                self.results.retain_mut(|r| {
                                    r.should_remain(process, address, self.filter, value)
                                });
                                self.scanner_status = Some(
                                    format!("Finished in: {:.2}", time.elapsed().as_secs_f32())
                                        .into(),
                                );
                            }

                            if ui.button("Clear results").clicked() {
                                self.results.clear();
                                self.scanner_status = None;
                            }

                            // This looks a bit nasty but *shrug*
                            let mut as_hex = matches!(self.display, DisplayMode::Hex);
                            if !matches!(self.field_kind, FieldKind::F32 | FieldKind::F64)
                                && ui.checkbox(&mut as_hex, "Show as hex").changed()
                            {
                                if as_hex {
                                    self.display = DisplayMode::Hex;
                                } else {
                                    self.display = DisplayMode::Normal;
                                }
                            }

                            if !self.scanner.active() {
                                ui.separator();
                                ui.label(format!("Total count: {}", self.results.len()));
                            }

                            Ok(())
                        })
                        .inner;
                    inner?;

                    ui.separator();

                    self.display_results(process, ui);
                }

                Ok(())
            })
            .and_then(|v| v.inner)
            .transpose()
    }

    fn display_results(&mut self, process: &Process, ui: &mut Ui) {
        const DATA_HEIGHT: f32 = 14.;
        ui.style_mut().override_font_id = Some(FontId::monospace(DATA_HEIGHT));

        let Some(address) = self.base_address
            .value()
            .and_then(|v| v.ok())
            .cloned()
        else {
            ui.heading("Invalid base address");
            return;
        };

        let levels = *self.max_levels.value().unwrap().unwrap();
        let w = ui.available_width() / (levels + 2) as f32 - 4.;

        TableBuilder::new(ui)
            .striped(true)
            .columns(Column::initial(w).resizable(true), levels + 1)
            .column(Column::remainder())
            .header(16., |mut row| {
                for i in 1..=levels {
                    row.col(|ui| _ = ui.label(format!("{i}")));
                }

                row.col(|ui| _ = ui.label("Previous"));
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

                    // Display last value
                    row.col(|ui| _ = ui.label(self.display.format(result.last_value)));

                    let mut address = address;
                    let mut buf = [0; 8];
                    for offset in result.parent_offsets.iter() {
                        process.read(address + offset, &mut buf[..]);
                        address = usize::from_ne_bytes(buf);
                    }

                    process.read(address + result.offset, &mut buf[..]);
                    address = usize::from_ne_bytes(buf);

                    process.read(address, &mut buf[..]);

                    // Display current value
                    let current = bytes_to_value(&buf, result.last_value.kind());
                    let text = self.display.format(current);
                    row.col(|ui| {
                        if current != result.last_value {
                            ui.label(RichText::new(text).color(Color32::KHAKI));
                        } else {
                            ui.label(text);
                        }
                    });
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
            offsets: Arc::default(),
            struct_size,
            alignment,
            address,
            depth,
            value,
        })
    }
}
