use crate::prelude::{Slice, Track};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarLayer {
    pub track: Option<Arc<Track>>,
    pub rounds: Option<Vec<u16>>,
    pub slices: Vec<Arc<Slice>>,
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
impl From<Vec<Arc<Slice>>> for BarLayer {
    fn from(v: Vec<Arc<Slice>>) -> Self {
        Self {
            track: None,
            rounds: None,
            slices: v,
        }
    }
}
impl From<(&Arc<Track>, Vec<Arc<Slice>>)> for BarLayer {
    fn from(v: (&Arc<Track>, Vec<Arc<Slice>>)) -> Self {
        Self {
            track: Some(v.0.clone()),
            rounds: None,
            slices: v.1,
        }
    }
}
impl From<(Vec<u16>, Vec<Arc<Slice>>)> for BarLayer {
    fn from(v: (Vec<u16>, Vec<Arc<Slice>>)) -> Self {
        Self {
            track: None,
            rounds: Some(v.0),
            slices: v.1,
        }
    }
}
impl From<(&Arc<Track>, Vec<u16>, Vec<Arc<Slice>>)> for BarLayer {
    fn from(v: (&Arc<Track>, Vec<u16>, Vec<Arc<Slice>>)) -> Self {
        Self {
            track: Some(v.0.clone()),
            rounds: Some(v.1),
            slices: v.2,
        }
    }
}
impl From<Vec<Arc<BarLayer>>> for Bar {
    fn from(v: Vec<Arc<BarLayer>>) -> Self {
        Self { layers: v }
    }
}
