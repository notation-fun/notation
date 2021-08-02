use std::sync::{Arc, Weak};

use notation_proto::prelude::{Entry, ProtoEntry};

#[derive(Debug)]
pub struct ModelEntry {
    pub entries: Arc<Vec<Arc<ProtoEntry>>>,
    pub index: usize,
    pub value: Arc<ProtoEntry>,
}
impl ModelEntry {
    pub fn new(
        entries: Arc<Vec<Arc<ProtoEntry>>>,
        index: usize,
        value: Arc<ProtoEntry>,
    ) -> Self {
        Self { entries, index, value }
    }
}
impl Entry for ModelEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.value.duration()
    }
}
impl ModelEntry {
    pub fn prev(&self) -> Option<&Arc<ProtoEntry>> {
        self.entries.get(self.index - 1)
    }
    pub fn next(&self) -> Option<&Arc<ProtoEntry>> {
        self.entries.get(self.index + 1)
    }
    pub fn prev_as_mark(&self) -> Option<&String> {
        self.prev()
            .and_then(|x| x.as_mark())
    }
}