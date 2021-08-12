use fehler::throws;
use notation_proto::prelude::{Fretboard4, Fretboard6, HandShape4, HandShape6};

use crate::prelude::{BarLane, LaneKind, ParseError, Slice, SliceEntry, Track};
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarLayer {
    pub track: Arc<Track>,
    pub slices: Vec<Arc<Slice>>,
}
#[derive(Debug)]
pub struct Bar {
    pub layers: Vec<Arc<BarLayer>>,
    pub lanes: Vec<Arc<BarLane>>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayer>({} S:{})", self.track, self.slices.len())
    }
}
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>(Y:{} N:{})",
            stringify!(Bar),
            self.layers.len(),
            self.lanes.len()
        )
    }
}
impl BarLayer {
    pub fn new(track: Arc<Track>, slices: Vec<Arc<Slice>>) -> Self {
        Self { track, slices }
    }
}
impl TryFrom<(notation_proto::prelude::BarLayer, &Vec<Arc<Track>>)> for BarLayer {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::BarLayer, &Vec<Arc<Track>>)) -> Self {
        let track =
            v.1.iter()
                .find(|x| x.id == v.0.track)
                .cloned()
                .ok_or(ParseError::TrackNotFound(v.0.track))?;
        let mut slices = Vec::new();
        for slice in v.0.slices {
            slices.push(Slice::new_arc(&track, slice.begin, slice.end, slice.rounds));
        }
        Self::new(track, slices)
    }
}
impl TryFrom<(notation_proto::prelude::Bar, &Vec<Arc<Track>>)> for Bar {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Bar, &Vec<Arc<Track>>)) -> Self {
        let mut layers = Vec::new();
        for layer in v.0.layers {
            layers.push(BarLayer::try_from((layer, v.1)).map(Arc::new)?);
        }
        let mut lanes = Vec::new();
        for layer in layers.iter() {
            for slice in layer.slices.iter() {
                if let Some(lane) = BarLane::try_from_slice(slice.clone()) {
                    lanes.push(Arc::new(lane));
                }
            }
        }
        Self { layers, lanes }
    }
}

impl Bar {
    pub fn get_lane_of_kind(&self, kind: LaneKind) -> Option<Arc<BarLane>> {
        for lane in self.lanes.iter() {
            if lane.kind == kind {
                return Some(lane.clone());
            }
        }
        None
    }
}

macro_rules! impl_get_fretted_shape {
    ($name:ident, $strings:literal, $as_fretted:ident, $get_fretboard:ident, $fretboard:ident, $hand_shape:ident) => {
        impl Bar {
            pub fn $name(&self, entry: &SliceEntry) -> Option<($fretboard, $hand_shape)> {
                if let Some(shapes_lane) = self.get_lane_of_kind(LaneKind::Shapes) {
                    for lane_entry in shapes_lane.slice.entries.iter() {
                        if entry.props.in_bar_pos
                            > lane_entry.props.in_bar_pos + lane_entry.model().props.tied_units
                        {
                            continue;
                        }
                        if entry.props.in_bar_pos < lane_entry.props.in_bar_pos {
                            break;
                        }
                        if let Some(fretted_entry) = lane_entry.model().$as_fretted() {
                            if let Some((shape, _duration)) = fretted_entry.as_shape() {
                                if let Some(fretboard) = shapes_lane.slice.track.$get_fretboard() {
                                    return Some((fretboard, shape.clone()));
                                } else {
                                    return None;
                                }
                            }
                        }
                    }
                }
                None
            }
        }
    };
}

impl_get_fretted_shape!(
    get_fretted_shape6,
    6,
    as_fretted6,
    get_fretboard6,
    Fretboard6,
    HandShape6
);
impl_get_fretted_shape!(
    get_fretted_shape4,
    4,
    as_fretted4,
    get_fretboard4,
    Fretboard4,
    HandShape4
);
