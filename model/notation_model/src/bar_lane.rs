use std::fmt::Display;
use std::sync::{Arc, Weak};

use notation_proto::prelude::Units;

use crate::prelude::{LaneEntry, LaneKind, Slice, Tab, TabBar, TabBarProps, Track, TrackProps, LaneEntryProps};

#[derive(Copy, Clone, Debug, Default)]
pub struct BarLaneProps {
    pub index: usize,
    pub track: TrackProps,
}

#[derive(Clone, Debug)]
pub struct BarLane {
    pub bar: Weak<TabBar>,
    pub kind: LaneKind,
    pub track: Arc<Track>,
    pub entries: Vec<Arc<LaneEntry>>,
    pub props: BarLaneProps,
}
impl Display for BarLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<BarLane>({} E:{})",
            self.id(),
            self.entries.len()
        )
    }
}

impl BarLane {
    pub fn id(&self) -> String {
        self.kind_id(self.kind)
    }
    pub fn kind_id(&self, kind: LaneKind) -> String {
        format!("{}:{}", self.track.id, kind)
    }
    pub fn order(&self) -> (usize, LaneKind) {
        (self.track.props.index, self.kind)
    }
    pub fn try_new_arc(
        bar: Weak<TabBar>,
        index: usize,
        track: &Arc<Track>,
        slice: Slice,
    ) -> Option<Arc<Self>> {
        let model_entries = track.get_entries(&slice.begin, &slice.end);
        let kind = LaneKind::of_entries(&track.kind, &model_entries);
        if !kind.is_none() {
            Some(Arc::<Self>::new_cyclic(|weak_self| {
                let props = BarLaneProps {
                    index,
                    track: track.props,
                };
                let entries = LaneEntry::new_entries(model_entries, weak_self, slice);
                Self {
                    bar,
                    kind,
                    track: track.clone(),
                    entries,
                    props,
                }
            }))
        } else {
            None
        }
    }
    pub fn merge_lane(&self, lane: &BarLane) -> Arc<Self> {
        Arc::<Self>::new_cyclic(|weak_self| {
            let mut entries = Vec::new();
            for entry in self.entries.iter() {
                entries.push(entry.clone());
            }
            for entry in lane.entries.iter() {
                entries.push(entry.clone());
            }
            entries.sort_by(|a, b| {
                a.props.in_bar_pos.cmp(&b.props.in_bar_pos)
            });
            let entries = entries.iter().enumerate().map(|(index, entry)| {
                let merged_entry = LaneEntry {
                    props: LaneEntryProps {
                        slice: entry.props.slice.clone(),
                        index,
                        ..entry.props
                    },
                    lane: weak_self.clone(),
                    model: entry.model.clone(),
                };
                Arc::new(merged_entry)
            }).collect();
            Self {
                bar: self.bar.clone(),
                kind: self.kind,
                track: self.track.clone(),
                entries,
                props: self.props,
            }
        })
    }
    pub fn bar(&self) -> Option<Arc<TabBar>> {
        self.bar.upgrade().map(|x| x.clone())
    }
    pub fn tab(&self) -> Option<Arc<Tab>> {
        self.bar().and_then(|x| x.tab())
    }
    pub fn bar_props(&self) -> TabBarProps {
        self.bar().map(|x| x.props).unwrap_or_default()
    }
    pub fn get_entry_at<T, F: Fn(&LaneEntry) -> Option<T>>(
        &self,
        in_bar_pos: Units,
        predicate: &F,
    ) -> Option<T> {
        for entry in self.entries.iter() {
            if in_bar_pos < entry.props.in_bar_pos {
                break;
            } else {
                if in_bar_pos < entry.props.in_bar_pos + entry.model().props.tied_units {
                    if let Some(result) = predicate(entry.as_ref()) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }
    pub fn get_next_entry<T, F: Fn(&LaneEntry) -> Option<T>>(
        &self,
        in_bar_pos: Units,
        predicate: &F,
    ) -> Option<T> {
        self.get_entry(&|x: &LaneEntry| {
            if x.props.in_bar_pos.is_bigger_than(&in_bar_pos) {
                predicate(x)
            } else {
                None
            }
        })
    }
}
