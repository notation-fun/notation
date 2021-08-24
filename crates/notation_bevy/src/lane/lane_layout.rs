use std::fmt::Display;
use std::sync::Arc;
use bevy::prelude::*;
use notation_model::prelude::{BarLane, LaneKind, TrackProps};

#[derive(Clone, Debug)]
pub struct LaneLayoutData {
    pub index: usize,
    pub track_id: String,
    pub track_props: TrackProps,
    pub lane_kind: LaneKind,
    pub height: f32,
    pub margin: f32,
    pub lane: Option<Arc<BarLane>>,
    pub visible: bool,
}
impl LaneLayoutData {
    pub fn new(index: usize, lane: &BarLane, height: f32, margin: f32) -> Self {
        Self {
            index,
            track_id: lane.track.id.clone(),
            track_props: lane.track.props,
            lane_kind: lane.kind,
            height,
            margin,
            lane: None,
            visible: false,
        }
    }
    pub fn id(&self) -> String {
        format!("{}:{}", self.track_id, self.lane_kind)
    }
    pub fn order(&self) -> (usize, LaneKind) {
        (self.track_props.index, self.lane_kind)
    }
    pub fn is_ghose(&self) -> bool {
        self.lane.is_none()
    }
}
impl Display for LaneLayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lane = self.lane.as_ref().map(|x| x.to_string()).unwrap_or_default();
        write!(
            f,
            "<LaneLayoutData>({} {:?} {}: {}, V:{})",
            self.track_id,
            self.track_props,
            self.lane_kind,
            lane,
            self.visible,
        )
    }
}