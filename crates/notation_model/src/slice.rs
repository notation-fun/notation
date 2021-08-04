use fehler::{throw, throws};
use notation_proto::prelude::{SliceBegin, SliceEnd};
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{LaneKind, ModelEntry, ParseError, Track};

#[derive(Debug)]
pub struct Slice {
    pub track: Arc<Track>,
    pub begin: SliceBegin,
    pub end: SliceEnd,
    pub rounds: Option<Vec<usize>>,
    pub entries: Vec<Arc<ModelEntry>>,
}

impl Slice {
    pub fn new(
        track: &Arc<Track>,
        begin: SliceBegin,
        end: SliceEnd,
        rounds: Option<Vec<usize>>,
    ) -> Self {
        let entries = track.get_entries(&begin, &end);
        Self {
            track: track.clone(),
            begin,
            end,
            rounds,
            entries,
        }
    }
    pub fn new_arc(
        track: &Arc<Track>,
        begin: SliceBegin,
        end: SliceEnd,
        rounds: Option<Vec<usize>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(track, begin, end, rounds))
    }
    pub fn calc_lane_kind(&self) -> Option<LaneKind> {
        for entry in self.entries.iter() {
            if let Some(lane) = LaneKind::calc_lane_kind(&self.track.kind, &entry.value) {
                return Some(lane);
            }
        }
        None
    }
}
impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Slice>({} {}-{}",
            self.track.id,
            self.begin,
            self.end,
        )?;
        if let Some(ref rounds) = self.rounds {
            write!(f, " R:{:?}", rounds)?;
        }
        write!(f, ")")
    }
}
impl TryFrom<(&Arc<Track>, notation_proto::prelude::Slice)> for Slice {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (&Arc<Track>, notation_proto::prelude::Slice)) -> Self {
        Self::new(v.0, v.1.begin, v.1.end, v.1.rounds)
    }
}
