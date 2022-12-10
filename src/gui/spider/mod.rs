mod gui;
pub use gui::*;
mod scanner;
pub use scanner::*;

use crate::{field::FieldKind, process::Process, value::Value};
use std::sync::Arc;

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

struct SearchOptions {
    offsets: Arc<Vec<usize>>,
    struct_size: usize,
    alignment: usize,
    address: usize,
    depth: usize,
    value: Value,
}

#[derive(Debug)]
struct SearchResult {
    // This should optimize memory usage for large amount of offsets,
    // We aren't modifying them anyways.
    parent_offsets: Arc<Vec<usize>>,
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
        let result = match filter {
            FilterMode::Less => current_value < new_value,
            FilterMode::LessEq => current_value <= new_value,
            FilterMode::Greater => current_value > new_value,
            FilterMode::GreaterEq => current_value >= new_value,
            FilterMode::Equal => current_value == new_value,
            FilterMode::NotEqual => current_value != new_value,
            FilterMode::Changed => current_value != self.last_value,
            FilterMode::Unchanged => current_value == self.last_value,
        };

        self.last_value = current_value;
        result
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
