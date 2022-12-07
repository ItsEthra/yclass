use crate::field::{Field, HexField};
use std::iter::repeat_with;

pub type ClassId = usize;

pub struct Class {
    id: ClassId,
    pub name: String,
    pub fields: Vec<Box<dyn Field>>,
}

impl Class {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            fields: repeat_with(|| Box::new(HexField::<8>::new()) as Box<dyn Field>)
                .take(20)
                .collect(),
        }
    }

    fn empty(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            fields: vec![],
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct ClassList {
    classes: Vec<Class>,
    selected: Option<usize>,
}

impl Default for ClassList {
    fn default() -> Self {
        Self {
            classes: vec![Class::new(0, "FirstClass".into())],
            selected: Some(0),
        }
    }
}

impl ClassList {
    pub const EMPTY: Self = Self {
        selected: None,
        classes: vec![],
    };

    pub fn classes(&self) -> &[Class] {
        &self.classes[..]
    }

    pub fn classes_mut(&mut self) -> &mut [Class] {
        &mut self.classes[..]
    }

    pub fn add_empty_class(&mut self, name: String) -> usize {
        let id = fastrand::usize(..);
        self.classes.push(Class::empty(id, name));
        id
    }

    pub fn add_class(&mut self, name: String) -> usize {
        let id = fastrand::usize(..);
        self.classes.push(Class::new(id, name));

        id
    }

    pub fn add_class_with_id(&mut self, name: String, id: usize) {
        self.classes.push(Class::new(id, name));
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected
    }

    pub fn by_id(&self, id: usize) -> Option<&Class> {
        self.classes.iter().find(|c| c.id == id)
    }

    pub fn by_id_mut(&mut self, id: usize) -> Option<&mut Class> {
        self.classes.iter_mut().find(|c| c.id == id)
    }

    pub fn by_name(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }

    pub fn delete_by_id(&mut self, id: usize) {
        self.classes.retain(|c| c.id != id);
    }

    pub fn selected_class(&self) -> Option<&Class> {
        self.selected
            .and_then(|i| self.classes.iter().find(|c| c.id == i))
    }
}
