use crate::prelude::{Slice, Track};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BarLayer {
    pub id: String,
    pub slices: Vec<Slice>,
    pub track: Option<String>,
    pub rounds: Option<Vec<usize>>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bar {
    pub layers: Vec<String>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayer>(")?;
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
        id: String,
        slices: Vec<Slice>,
        track: Option<String>,
        rounds: Option<Vec<usize>>,
    ) -> Self {
        Self {
            id,
            slices,
            track,
            rounds,
        }
    }
}
impl From<(String, Vec<Slice>)> for BarLayer {
    fn from(v: (String, Vec<Slice>)) -> Self {
        Self::new(v.0, v.1, None, None)
    }
}
impl From<(String, Vec<Slice>, &Track)> for BarLayer {
    fn from(v: (String, Vec<Slice>, &Track)) -> Self {
        Self::new(v.0, v.1, Some(v.2.id.clone()), None)
    }
}
impl From<(String, Vec<Slice>, Vec<usize>)> for BarLayer {
    fn from(v: (String, Vec<Slice>, Vec<usize>)) -> Self {
        Self::new(v.0, v.1, None, Some(v.2))
    }
}
impl From<(String, Vec<Slice>, &Track, Vec<usize>)> for BarLayer {
    fn from(v: (String, Vec<Slice>, &Track, Vec<usize>)) -> Self {
        Self::new(v.0, v.1, Some(v.2.id.clone()), Some(v.3))
    }
}
impl From<Vec<String>> for Bar {
    fn from(v: Vec<String>) -> Self {
        Self { layers: v }
    }
}
impl From<Vec<&str>> for Bar {
    fn from(v: Vec<&str>) -> Self {
        Self {
            layers: v.iter().map(|x| x.to_string()).collect(),
        }
    }
}
