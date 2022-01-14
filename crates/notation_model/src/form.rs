use std::fmt::Display;
use std::sync::Arc;

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
impl Form {
    pub fn new(add_ready_section: bool, proto: notation_proto::prelude::Form, tab_section: &Vec<Arc<Section>>) -> Self {
        let mut sections = Vec::new();
        let mut add_section =
            |section_id: String| match tab_section.iter().find(|x| x.id == section_id).cloned() {
                Some(section) => sections.push(section),
                None => println!("Form::from(), bad section: {}", section_id),
            };
        if add_ready_section {
            add_section(notation_proto::prelude::Section::READY_ID.to_string());
        }
        for section_id in proto.sections {
            add_section(section_id);
        }
        Self { sections }
    }
}
