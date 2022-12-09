use crate::{field::FieldKind, state::StateRef, value::Value};
use eframe::egui::Context;

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

pub struct SpiderWindow {
    state: StateRef,
    shown: bool,
}

impl SpiderWindow {
    pub fn new(state: StateRef) -> Self {
        Self {
            state,
            shown: false,
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn show(&mut self, ctx: &Context) {}
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
