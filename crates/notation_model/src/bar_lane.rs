use std::fmt::Display;
use std::sync::{Arc, Weak};

use crate::prelude::{LaneEntry, LaneKind, Slice, Tab, TabBar, TabBarProps, Track};

#[derive(Copy, Clone, Debug, Default)]
pub struct BarLaneProps {
    pub index: usize,
}

#[derive(Debug)]
pub struct BarLane {
    pub bar: Weak<TabBar>,
    pub kind: LaneKind,
    pub track: Arc<Track>,
    pub slice: Slice,
    pub entries: Vec<Arc<LaneEntry>>,
    pub props: BarLaneProps,
}
impl Display for BarLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<BarLane>({} {}, E:{})",
            self.id(),
            self.slice,
            self.entries.len()
        )
    }
}

impl BarLane {
    pub fn id(&self) -> String {
        format!("{}:{}", self.track.id, self.kind)
    }
    pub fn try_new_arc(
        bar: Weak<TabBar>,
        index: usize,
        track: &Arc<Track>,
        slice: Slice
    ) -> Option<Arc<Self>> {
        let model_entries = track.get_entries(&slice.begin, &slice.end);
        if let Some(kind) = LaneKind::of_entries(&track.kind, &model_entries) {
            Some(Arc::<Self>::new_cyclic(|weak_self| {
                let props = BarLaneProps {
                    index,
                };
                let entries = LaneEntry::new_entries(model_entries, weak_self);
                Self {
                    bar,
                    kind,
                    track: track.clone(),
                    slice,
                    entries,
                    props,
                }
            }))
        } else {
            None
        }
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
}
