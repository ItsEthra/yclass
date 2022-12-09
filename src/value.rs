use crate::field::FieldKind;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Value {
    pub const fn kind(&self) -> FieldKind {
        match self {
            Value::U8(_) => FieldKind::U8,
            Value::I8(_) => FieldKind::I8,
            Value::U16(_) => FieldKind::U16,
            Value::I16(_) => FieldKind::I16,
            Value::U32(_) => FieldKind::U32,
            Value::I32(_) => FieldKind::I32,
            Value::U64(_) => FieldKind::U64,
            Value::I64(_) => FieldKind::I64,
            Value::F32(_) => FieldKind::F32,
            Value::F64(_) => FieldKind::F64,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            _ => panic!("Comparing different value types"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::U8(l0), Self::U8(r0)) => l0.partial_cmp(r0),
            (Self::I8(l0), Self::I8(r0)) => l0.partial_cmp(r0),
            (Self::U16(l0), Self::U16(r0)) => l0.partial_cmp(r0),
            (Self::I16(l0), Self::I16(r0)) => l0.partial_cmp(r0),
            (Self::U32(l0), Self::U32(r0)) => l0.partial_cmp(r0),
            (Self::I32(l0), Self::I32(r0)) => l0.partial_cmp(r0),
            (Self::U64(l0), Self::U64(r0)) => l0.partial_cmp(r0),
            (Self::I64(l0), Self::I64(r0)) => l0.partial_cmp(r0),
            (Self::F32(l0), Self::F32(r0)) => l0.partial_cmp(r0),
            (Self::F64(l0), Self::F64(r0)) => l0.partial_cmp(r0),
            _ => panic!("Comparing different value types"),
        }
    }
}

macro_rules! impl_traits {
    ($($var:ident, $type:ty),*) => {
        $(
            impl From<$type> for Value {
                fn from(v: $type) -> Self {
                    Self::$var(v)
                }
            }
        )*
    };
}

impl_traits!(
    U8, u8, I8, i8, U16, u16, I16, i16, U32, u32, I32, i32, U64, u64, I64, i64, F32, f32, F64, f64
);
