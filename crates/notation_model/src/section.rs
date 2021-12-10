use fehler::throws;
use std::fmt::Display;
use std::sync::{Arc, Weak};

use crate::prelude::{Bar, ParseError, SectionKind, Tab, Track};

#[derive(Debug)]
pub struct Section {
    pub tab: Weak<Tab>,
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
    pub fn new(
        tab: Weak<Tab>,
        index: usize,
        kind: SectionKind,
        id: String,
        bars: Vec<Arc<Bar>>,
    ) -> Self {
        Self {
            tab,
            id,
            kind,
            bars,
            index,
        }
    }
}

impl Section {
    #[throws(ParseError)]
    pub fn try_new(
        tab: Weak<Tab>,
        index: usize,
        proto: notation_proto::prelude::Section,
        tracks: &Vec<Arc<Track>>,
    ) -> Self {
        let mut bars = Vec::new();
        for (bar_index, bar) in proto.bars.into_iter().enumerate() {
            bars.push(Bar::try_new(bar_index, bar, tracks).map(Arc::new)?);
        }
        Self::new(tab, index, proto.kind, proto.id, bars)
    }
}
