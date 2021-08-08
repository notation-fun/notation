use crate::prelude::Slice;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BarLayer {
    pub track: String,
    pub slices: Vec<Slice>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bar {
    pub layers: Vec<BarLayer>,
}
impl Display for BarLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayer>({} S:{})", self.track, self.slices.len())
    }
}
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>(L:{})", stringify!(Bar), self.layers.len())
    }
}
impl BarLayer {
    pub fn new(track: String, slices: Vec<Slice>) -> Self {
        Self { track, slices }
    }
}
impl From<(String, Vec<Slice>)> for BarLayer {
    fn from(v: (String, Vec<Slice>)) -> Self {
        Self::new(v.0, v.1)
    }
}
impl From<(&str, Vec<Slice>)> for BarLayer {
    fn from(v: (&str, Vec<Slice>)) -> Self {
        Self::new(String::from(v.0), v.1)
    }
}
impl From<Vec<BarLayer>> for Bar {
    fn from(v: Vec<BarLayer>) -> Self {
        Self { layers: v }
    }
}
