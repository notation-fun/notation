use fehler::throws;
use notation_proto::prelude::{SliceBegin, SliceEnd};

use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{LaneKind, ParseError, SliceEntry, Track};

#[derive(Debug)]
pub struct Slice {
    pub track: Arc<Track>,
    pub begin: SliceBegin,
    pub end: SliceEnd,
    pub rounds: Option<Vec<usize>>,
    pub entries: Vec<Arc<SliceEntry>>,
}

impl Slice {
    pub fn new_arc(
        track: &Arc<Track>,
        begin: SliceBegin,
        end: SliceEnd,
        rounds: Option<Vec<usize>>,
    ) -> Arc<Self> {
        Arc::<Self>::new_cyclic(|weak_self| {
            let entries = SliceEntry::new_entries(track.get_entries(&begin, &end), weak_self);
            Self {
                track: track.clone(),
                begin,
                end,
                rounds,
                entries,
            }
        })
    }
    pub fn calc_lane_kind(&self) -> Option<LaneKind> {
        for entry in self.entries.iter() {
            if let Some(lane) = LaneKind::calc_lane_kind(&self.track.kind, &entry.model.proto) {
                return Some(lane);
            }
        }
        None
    }
}
impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Slice>({} {}-{}", self.track.id, self.begin, self.end,)?;
        if let Some(ref rounds) = self.rounds {
            write!(f, " R:{:?}", rounds)?;
        }
        write!(f, ")")
    }
}
