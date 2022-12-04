use crate::field::FieldKind;

mod rust;
pub use rust::*;
mod cpp;
pub use cpp::*;

pub trait Generator {
    fn begin_class(&mut self, name: &str);
    fn end_class(&mut self);

    fn add_field(&mut self, name: &str, kind: FieldKind, metadata: Option<&str>);
    fn add_offset(&mut self, offset: usize);

    fn finilize(&mut self) -> String;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AvailableGenerator {
    #[default]
    Rust,
    Cpp,
}

impl AvailableGenerator {
    pub const ALL: &[AvailableGenerator] = &[AvailableGenerator::Rust, AvailableGenerator::Cpp];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Cpp => "C++",
        }
    }

    pub fn generator(&self) -> Box<dyn Generator> {
        match self {
            Self::Rust => Box::new(RustGenerator::default()),
            Self::Cpp => Box::new(CppGenerator::default()),
        }
    }
}
