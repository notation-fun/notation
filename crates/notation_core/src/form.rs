use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::prelude::{Line, Slice, Signature, Key, Scale};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TrackKind {
    Vocal,
    Piano,
    Guitar,
    Bass,
    Drums,
    Custom(String, u8),
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub kind: TrackKind,
}

#[derive(Clone)]
pub struct BarLayer {
    pub track: Rc<Track>,
    pub slices: Vec<Rc<Slice>>,
}

#[derive(Clone)]
pub struct Bar {
    pub layers: Vec<Rc<BarLayer>>,
}

#[derive(Clone)]
pub struct Section {
    pub name: String,
    pub bars: Vec<Rc<Bar>>,
}

// https://hellomusictheory.com/learn/form/
#[derive(Clone)]
pub enum Form {
    Strophic,
    Binary,
    Ternary,
    Rondo,
    Medley,
    Variational,
    Sonata,
}

#[derive(Clone)]
pub struct Composition {
    pub key: Key,
    pub scale: Scale,
    pub signature: Signature,
    pub form: Option<Form>,
    pub lines: Vec<Rc<Line>>,
    pub tracks: Vec<Rc<Track>>,
    pub sections: Vec<Rc<Section>>,
}

pub type Song = Composition;