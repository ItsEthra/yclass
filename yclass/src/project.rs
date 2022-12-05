/// This module contains structures that serialize/deserialize project data(i.e. classes).
use crate::{
    class::{Class, ClassList},
    field::{allocate_padding, CodegenData, Field, FieldKind, PointerField},
    generator::Generator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataField {
    name: String,
    offset: usize,
    kind: FieldKind,
    metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataClass {
    name: String,
    fields: Vec<DataField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectData {
    classes: Vec<DataClass>,
}

#[derive(Default, Clone)]
struct ProjectDataGenerator {
    classes: Vec<DataClass>,
    offset: usize,
    last_offset: usize,
}

impl Generator for &mut ProjectDataGenerator {
    fn begin_class(&mut self, name: &str) {
        self.classes.push(DataClass {
            name: name.into(),
            fields: vec![],
        });
    }

    fn add_field(&mut self, name: &str, kind: FieldKind, metadata: Option<&str>) {
        let size = kind.size();

        self.classes.last_mut().unwrap().fields.push(DataField {
            metadata: metadata.map(|s| s.to_owned()),
            name: name.to_owned(),
            offset: self.offset,
            kind,
        });

        self.offset += size;
        self.last_offset = self.offset;
    }

    fn add_offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn end_class(&mut self) {
        self.offset = 0;
        self.last_offset = 0;
    }

    fn finilize(&mut self) -> String {
        unimplemented!()
    }
}

impl ProjectData {
    pub fn store(classes: &[Class]) -> Self {
        let mut datagen = ProjectDataGenerator::default();
        let dynam = &mut &mut datagen as &mut dyn Generator;
        let data = CodegenData { classes };

        for class in classes {
            dynam.begin_class(&class.name);
            for f in class.fields.iter() {
                f.codegen(dynam, &data);
            }
            dynam.end_class();
        }

        Self {
            classes: datagen.classes,
        }
    }

    pub fn load(self) -> ClassList {
        let mut list = ClassList::EMPTY;

        self.classes.into_iter().for_each(|mut class| {
            class.fields.sort_by_key(|f| f.offset);

            let cid = list.add_empty_class(class.name);
            let mut current_offset = 0;

            for DataField {
                offset: field_offset,
                name,
                kind,
                metadata,
            } in class.fields
            {
                let class = list.by_id_mut(cid).unwrap();

                if field_offset > current_offset {
                    class
                        .fields
                        .extend(allocate_padding(field_offset - current_offset));
                }

                match kind {
                    FieldKind::Ptr => {
                        let refname = metadata.unwrap();
                        if let Some(refclass) = list.by_name(&refname) {
                            let refid = refclass.id();
                            let class = list.by_id_mut(cid).unwrap();
                            class
                                .fields
                                .push(Box::new(PointerField::new_with_class_id(name, refid))
                                    as Box<dyn Field>);
                        } else {
                            let new_cid = list.add_class(refname);
                            let class = list.by_id_mut(cid).unwrap();
                            class
                                .fields
                                .push(Box::new(PointerField::new_with_class_id(name, new_cid))
                                    as Box<dyn Field>);
                        }
                    }
                    other => class.fields.push(other.into_field(Some(name))),
                }

                current_offset = field_offset + kind.size();
            }
        });

        list
    }

    #[allow(clippy::inherent_to_string)]
    pub fn from_str(text: &str) -> Option<Self> {
        ron::from_str(text).ok()
    }

    pub fn to_string(&self) -> String {
        ron::to_string(self).unwrap()
    }
}
