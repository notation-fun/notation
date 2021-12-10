use std::fmt::Display;
use std::sync::{Arc};

use crate::section::Section;

#[derive(Debug)]
pub struct Form {
    pub sections: Vec<Arc<Section>>,
}
impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Form>(S:{})", self.sections.len())
    }
}
impl From<(notation_proto::prelude::Form, &Vec<Arc<Section>>)> for Form {
    fn from(v: (notation_proto::prelude::Form, &Vec<Arc<Section>>)) -> Self {
        let mut sections = Vec::new();
        let mut add_section = |section_id: String| {
            match v.1.iter().find(|x| x.id == section_id).cloned() {
                Some(section) => sections.push(section),
                None => println!("Form::from(), bad setion: {}", section_id),
            }
        };
        add_section(notation_proto::prelude::Section::REST_ID.to_string());
        for section_id in v.0.sections {
            add_section(section_id);
        }
        Self { sections }
    }
}
