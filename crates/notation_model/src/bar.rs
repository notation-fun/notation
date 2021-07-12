use fehler::throws;

use crate::prelude::{Line, ParseError, Slice, Track};
use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarLayer {
    pub key: String,
    pub slices: Vec<Arc<Slice>>,
    pub track: Option<Arc<Track>>,
    pub rounds: Option<Vec<u16>>,
}
#[derive(Debug)]
pub struct Bar {
    pub layers: Vec<Arc<BarLayer>>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>(", stringify!(BarLayer))?;
        if let Some(ref track) = self.track {
            write!(f, "{} ", track)?;
        }
        if let Some(ref rounds) = self.rounds {
            write!(f, "R:{:?}, ", rounds)?;
        }
        write!(f, "S:{})", self.slices.len())?;
        Ok(())
    }
}
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>(L:{})", stringify!(Bar), self.layers.len())
    }
}
impl BarLayer {
    pub fn new(
        key: String,
        slices: Vec<Arc<Slice>>,
        track: Option<Arc<Track>>,
        rounds: Option<Vec<u16>>,
    ) -> Self {
        Self {
            key,
            slices,
            track,
            rounds,
        }
    }
}
impl
    TryFrom<(
        notation_proto::prelude::BarLayer,
        &Vec<Arc<Line>>,
        &Vec<Arc<Track>>,
    )> for BarLayer
{
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(
        v: (
            notation_proto::prelude::BarLayer,
            &Vec<Arc<Line>>,
            &Vec<Arc<Track>>,
        ),
    ) -> Self {
        let mut slices = Vec::new();
        for slice in v.0.slices {
            slices.push(Slice::try_from((slice, v.1)).map(|x| Arc::new(x))?);
        }
        let track = match v.0.track {
            None => None,
            Some(track) => Some(
                v.2.iter()
                    .find(|x| x.key == track)
                    .map(|x| x.clone())
                    .ok_or(ParseError::TrackNotFound(track))?,
            ),
        };
        Self::new(v.0.key, slices, track, v.0.rounds)
    }
}
impl From<Vec<Arc<BarLayer>>> for Bar {
    fn from(v: Vec<Arc<BarLayer>>) -> Self {
        Self { layers: v }
    }
}
impl TryFrom<(notation_proto::prelude::Bar, &Vec<Arc<BarLayer>>)> for Bar {
    type Error = ParseError;

    #[throws(Self::Error)]
    fn try_from(v: (notation_proto::prelude::Bar, &Vec<Arc<BarLayer>>)) -> Self {
        let mut layers = Vec::new();
        for layer in v.0.layers {
            layers.push(
                v.1.iter()
                    .find(|x| x.key == layer)
                    .map(|x| x.clone())
                    .ok_or(ParseError::LayerNotFound(layer))?,
            );
        }
        Self::from(layers)
    }
}
