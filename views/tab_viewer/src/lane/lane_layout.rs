use std::fmt::Display;
use std::sync::{Arc, RwLock};
use edger_bevy_app::bevy_prelude::*;
use notation_model::prelude::{BarLane, LaneKind, TrackProps};

#[derive(Clone, Debug, Component)]
pub struct LaneLayoutData {
    pub track_id: String,
    pub track_props: TrackProps,
    pub lane_kind: LaneKind,
    pub height: f32,
    pub margin: f32,
    pub lane: Option<Arc<BarLane>>,
    pub visible: Arc<RwLock<bool>>,
}
impl LaneLayoutData {
    pub fn new(lane: &BarLane, height: f32, margin: f32) -> Self {
        Self {
            track_id: lane.track.id.clone(),
            track_props: lane.track.props,
            lane_kind: lane.kind,
            height,
            margin,
            lane: None,
            visible: Arc::new(RwLock::new(false)),
        }
    }
    pub fn new_virtual(lane: &BarLane, lane_kind: LaneKind, height: f32, margin: f32) -> Self {
        Self {
            track_id: lane.track.id.clone(),
            track_props: lane.track.props,
            lane_kind,
            height,
            margin,
            lane: None,
            visible: Arc::new(RwLock::new(false)),
        }
    }
    pub fn id(&self) -> String {
        format!("{}:{}", self.track_id, self.lane_kind)
    }
    pub fn order(&self) -> usize {
        self.track_props.index * LaneKind::LEN + self.lane_kind.order()
    }
    pub fn is_ghost(&self) -> bool {
        self.lane.is_none()
    }
    pub fn is_virtual(&self) -> bool {
        self.lane.is_some() && self.lane.as_ref().unwrap().kind != self.lane_kind
    }
    pub fn visible(&self) -> bool {
        self.height > 0.0 && *self.visible.read().unwrap()
    }
    pub fn set_visible(&self, visible: bool) {
        if self.height > 0.0 {
            *self.visible.write().unwrap() = visible;
        }
    }
}
impl Display for LaneLayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lane = self
            .lane
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or_default();
        write!(
            f,
            "<LaneLayoutData>({} {:?} {}: {}, H:{})",
            self.track_id, self.track_props, self.lane_kind, lane, self.height,
        )
    }
}
