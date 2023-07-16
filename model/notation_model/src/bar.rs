use fehler::throws;

use crate::prelude::{ParseError, Slice, Track};

use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarLayer {
    pub index: usize,
    pub track: Arc<Track>,
    pub slices: Vec<Slice>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<BarLayer>({} {} S:{})",
            self.index,
            self.track,
            self.slices.len()
        )
    }
}
impl BarLayer {
    pub fn new(index: usize, track: Arc<Track>, slices: Vec<Slice>) -> Self {
        Self {
            index,
            track,
            slices,
        }
    }
}
#[derive(Debug)]
pub struct Bar {
    pub index: usize,
    pub layers: Vec<Arc<BarLayer>>,
}
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Bar>({} L:{})", self.index, self.layers.len(),)
    }
}
impl BarLayer {
    #[throws(ParseError)]
    pub fn try_new(
        index: usize,
        proto: notation_proto::prelude::BarLayer,
        tracks: &Vec<Arc<Track>>,
    ) -> Self {
        let track = tracks
            .iter()
            .find(|x| x.id == proto.track)
            .cloned()
            .ok_or(ParseError::TrackNotFound(proto.track))?;
        Self::new(index, track, proto.slices)
    }
}
impl Bar {
    #[throws(ParseError)]
    pub fn try_new(
        index: usize,
        proto: notation_proto::prelude::Bar,
        tracks: &Vec<Arc<Track>>,
    ) -> Self {
        let mut layers = Vec::new();
        for (layer_index, layer) in proto.layers.into_iter().enumerate() {
            layers.push(BarLayer::try_new(layer_index, layer, tracks).map(Arc::new)?);
        }
        Self {
            index: index,
            layers,
        }
    }
}
