use fehler::throws;
use notation_proto::prelude::{Fretboard4, Fretboard6, HandShape4, HandShape6};

use crate::prelude::{BarLane, LaneKind, ParseError, Slice, LaneEntry, Track};
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarLayer {
    pub track: Arc<Track>,
    pub slices: Vec<Slice>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayer>({} S:{})", self.track, self.slices.len())
    }
}
impl BarLayer {
    pub fn new(track: Arc<Track>, slices: Vec<Slice>) -> Self {
        Self { track, slices }
    }
}
#[derive(Debug)]
pub struct Bar {
    pub layers: Vec<Arc<BarLayer>>,
}
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Bar>(L:{})",
            self.layers.len(),
        )
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
        Self::new(track, v.0.slices)
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
        Self { layers }
    }
}
