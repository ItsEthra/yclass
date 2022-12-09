use super::{BoolField, Field, FloatField, HexField, IntField, PointerField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum FieldKind {
    Unk8, Unk16, Unk32, Unk64,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Ptr,
    Bool,
}

impl FieldKind {
    pub const NAMED_VARIANTS: &[(FieldKind, &'static str)] = &[
        (Self::I8, "I8"),
        (Self::I16, "I16"),
        (Self::I32, "I32"),
        (Self::I64, "I64"),
        (Self::U8, "U8"),
        (Self::U16, "U16"),
        (Self::U32, "U32"),
        (Self::U64, "U64"),
        (Self::F32, "F32"),
        (Self::F64, "F64"),
    ];

    pub fn label(&self) -> Option<&'static str> {
        Self::NAMED_VARIANTS
            .iter()
            .find_map(|(v, s)| if v == self { Some(*s) } else { None })
    }
}

impl FieldKind {
    /// Returns size in bytes.
    pub fn size(&self) -> usize {
        match self {
            Self::Unk8 | Self::I8 | Self::U8 | Self::Bool => 1,
            Self::Unk16 | Self::I16 | Self::U16 => 2,
            Self::Unk32 | Self::I32 | Self::U32 | Self::F32 => 4,
            // TODO(ItsEthra): Pointer size is... sigh, different for 32-bit processes
            Self::Unk64 | Self::I64 | Self::U64 | Self::F64 | Self::Ptr => 8,
        }
    }

    pub fn into_field(self, name: Option<String>) -> Box<dyn Field> {
        match self {
            Self::Unk8 => Box::new(HexField::<1>::new()),
            Self::Unk16 => Box::new(HexField::<2>::new()),
            Self::Unk32 => Box::new(HexField::<4>::new()),
            Self::Unk64 => Box::new(HexField::<8>::new()),
            Self::I8 => Box::new(IntField::<1>::signed(name.unwrap_or_else(|| "int8".into()))),
            Self::I16 => Box::new(IntField::<2>::signed(
                name.unwrap_or_else(|| "int16".into()),
            )),
            Self::I32 => Box::new(IntField::<4>::signed(
                name.unwrap_or_else(|| "int32".into()),
            )),
            Self::I64 => Box::new(IntField::<8>::signed(
                name.unwrap_or_else(|| "int64".into()),
            )),
            Self::U8 => Box::new(IntField::<1>::unsigned(
                name.unwrap_or_else(|| "uint8".into()),
            )),
            Self::U16 => Box::new(IntField::<2>::unsigned(
                name.unwrap_or_else(|| "uint16".into()),
            )),
            Self::U32 => Box::new(IntField::<4>::unsigned(
                name.unwrap_or_else(|| "uint32".into()),
            )),
            Self::U64 => Box::new(IntField::<8>::unsigned(
                name.unwrap_or_else(|| "uint64".into()),
            )),
            Self::F32 => Box::new(FloatField::<4>::new(name.unwrap_or_else(|| "float".into()))),
            Self::F64 => Box::new(FloatField::<8>::new(
                name.unwrap_or_else(|| "double".into()),
            )),
            Self::Bool => Box::new(BoolField::new(name.unwrap_or_else(|| "boolean".into()))),
            Self::Ptr => Box::new(PointerField::new(name.unwrap_or_else(|| "pointer".into()))),
        }
    }
}
