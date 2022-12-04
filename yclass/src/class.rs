use crate::field::{Field, HexField};
use std::iter::repeat_with;

pub struct Class {
    pub name: String,
    pub fields: Vec<Box<dyn Field>>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: repeat_with(|| Box::new(HexField::<8>::new()) as Box<dyn Field>)
                .take(50)
                .collect(),
        }
    }
}
