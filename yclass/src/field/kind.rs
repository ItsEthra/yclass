use super::{Field, FloatField, HexField, IntField, PointerField};

#[derive(Debug, Clone, Copy, PartialEq)]
#[rustfmt::skip]
pub enum FieldKind {
    Unk8, Unk16, Unk32, Unk64,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Ptr,
}

impl FieldKind {
    /// Returns size in bytes.
    pub fn size(&self) -> usize {
        match self {
            Self::Unk8 | Self::I8 | Self::U8 => 1,
            Self::Unk16 | Self::I16 | Self::U16 => 2,
            Self::Unk32 | Self::I32 | Self::U32 | Self::F32 => 4,
            // TODO(ItsEthra): Pointer size is... sigh, different for 32-bit processes
            Self::Unk64 | Self::I64 | Self::U64 | Self::F64 | Self::Ptr => 8,
        }
    }

    pub fn into_field(self) -> Box<dyn Field> {
        match self {
            Self::Unk8 => Box::new(HexField::<1>::new()),
            Self::Unk16 => Box::new(HexField::<2>::new()),
            Self::Unk32 => Box::new(HexField::<4>::new()),
            Self::Unk64 => Box::new(HexField::<8>::new()),
            Self::I8 => Box::new(IntField::<1>::signed("int8".into())),
            Self::I16 => Box::new(IntField::<2>::signed("int16".into())),
            Self::I32 => Box::new(IntField::<4>::signed("int32".into())),
            Self::I64 => Box::new(IntField::<8>::signed("int64".into())),
            Self::U8 => Box::new(IntField::<1>::unsigned("uint8".into())),
            Self::U16 => Box::new(IntField::<2>::unsigned("uint16".into())),
            Self::U32 => Box::new(IntField::<4>::unsigned("uint32".into())),
            Self::U64 => Box::new(IntField::<8>::unsigned("uint64".into())),
            Self::F32 => Box::new(FloatField::<4>::new("float".into())),
            Self::F64 => Box::new(FloatField::<8>::new("double".into())),
            Self::Ptr => Box::new(PointerField::new()),
        }
    }
}
