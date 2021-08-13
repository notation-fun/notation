use fehler::throws;
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{Bar, ParseError, SectionKind, Track};

#[derive(Debug)]
pub struct Section {
    pub index: usize,
    pub kind: SectionKind,
    pub id: String,
    pub bars: Vec<Arc<Bar>>,
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Section>({} <{}> {} B:{})",
            self.index,
            self.kind,
            self.id,
            self.bars.len()
        )
    }
}
impl Section {
    pub fn new(index: usize, kind: SectionKind, id: String, bars: Vec<Arc<Bar>>) -> Self {
        Self {
            id,
            kind,
            bars,
            index,
        }
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

impl Section {
    #[throws(ParseError)]
    pub fn try_new(
        index: usize,
        proto: notation_proto::prelude::Section,
        tracks: &Vec<Arc<Track>>,
    ) -> Self {
        let mut bars = Vec::new();
        for (bar_index, bar) in proto.bars.into_iter().enumerate() {
            bars.push(Bar::try_new(bar_index, bar, tracks).map(Arc::new)?);
        }
        Self::new(index, proto.kind, proto.id, bars)
    }
}
