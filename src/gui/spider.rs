use super::{TextEditBind, TextEditFromStrBind};
use crate::{address::parse_address, field::FieldKind, state::StateRef, value::Value};
use eframe::egui::{ComboBox, Context, TextEdit, Ui, Window};

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

struct SearchResult {}

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

            results: vec![],
            shown: false,
            state,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
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
            depth,
            alignment,
            address,
            struct_size,
            value,
        })
    }

    pub fn show(&mut self, ctx: &Context) -> eyre::Result<Option<()>> {
        let shown = unsafe { &mut (*(self as *mut Self)).shown };
        Window::new("Structure spider")
            .open(shown)
            .show(ctx, |ui| {
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
                    }
                } else {
                }

                eyre::Result::Ok(())
            })
            .map(|v| v.inner)
            .flatten()
            .transpose()
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
