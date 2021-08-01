use fehler::throws;
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{Bar, BarLayer, ParseError, SectionKind};

#[derive(Debug)]
pub struct Section {
    pub id: String,
    pub kind: SectionKind,
    pub bars: Vec<Arc<Bar>>,
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Section>({} <{}> B:{})",
            self.id,
            self.kind,
            self.bars.len()
        )
    }
}
impl Section {
    pub fn new(id: String, kind: SectionKind, bars: Vec<Arc<Bar>>) -> Self {
        Self { id, kind, bars }
    }
}

#[derive(Debug)]
pub struct Form {
    pub sections: Vec<Arc<Section>>,
}
impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Form>(S:{})", self.sections.len())
    }
}
impl TryFrom<(notation_proto::prelude::Form, &Vec<Arc<Section>>)> for Form {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Form, &Vec<Arc<Section>>)) -> Self {
        let mut sections = Vec::new();
        for section in v.0.sections {
            sections.push(
                v.1.iter()
                    .find(|x| x.id == section)
                    .cloned()
                    .ok_or(ParseError::SectionNotFound(section))?,
            );
        }
        Self { sections }
    }
}

impl TryFrom<(notation_proto::prelude::Section, &Vec<Arc<BarLayer>>)> for Section {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Section, &Vec<Arc<BarLayer>>)) -> Self {
        let mut bars = Vec::new();
        for bar in v.0.bars {
            bars.push(Bar::try_from((bar, v.1)).map(Arc::new)?);
        }
        Self::new(v.0.id, v.0.kind, bars)
    }
}
