use fehler::throws;

use crate::prelude::{BarLane, ParseError, Slice, Track};
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
        write!(f, "<{}>(Y:{} N:{})", stringify!(Bar), self.layers.len(), self.lanes.len())
    }
}
impl BarLayer {
    pub fn new(
        track: Arc<Track>,
        slices: Vec<Arc<Slice>>,
    ) -> Self {
        Self {
            track,
            slices,
        }
    }
}
impl
    TryFrom<(
        notation_proto::prelude::BarLayer,
        &Vec<Arc<Track>>,
    )> for BarLayer
{
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(
        v: (
            notation_proto::prelude::BarLayer,
            &Vec<Arc<Track>>,
        ),
    ) -> Self {
        let track =
                v.1.iter()
                    .find(|x| x.id == v.0.track)
                    .cloned()
                    .ok_or(ParseError::TrackNotFound(v.0.track))?;
        let mut slices = Vec::new();
        for slice in v.0.slices {
            slices.push(Slice::try_from((&track, slice)).map(Arc::new)?);
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
        Self { layers, lanes, }
    }
}
