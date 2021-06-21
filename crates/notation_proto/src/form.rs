use std::rc::Rc;
use std::sync::Arc;

use serde::{Serialize, Deserialize};

use notation_core::prelude::{Signature, Key, Scale};
use crate::prelude::{RcLine, RcSlice, ArcLine, ArcSlice};

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

// https://hellomusictheory.com/learn/form/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Form {
    Strophic,
    Binary,
    Ternary,
    Rondo,
    Medley,
    Variational,
    Sonata,
}

macro_rules! impl_form {
    ($ref_type:ident, $line_name:ident, $slice_name:ident,
            $bar_layer_name:ident, $bar_name:ident,
            $section_name:ident, $tab_name:ident) => {
        #[derive(Clone)]
        pub struct $bar_layer_name {
            pub track: $ref_type<Track>,
            pub slices: Vec<$ref_type<$slice_name>>,
        }

        #[derive(Clone)]
        pub struct $bar_name {
            pub layers: Vec<$ref_type<$bar_layer_name>>,
        }

        #[derive(Clone)]
        pub struct $section_name {
            pub name: String,
            pub bars: Vec<$ref_type<$bar_name>>,
        }

        #[derive(Clone)]
        pub struct $tab_name {
            pub key: Key,
            pub scale: Scale,
            pub signature: Signature,
            pub form: Option<Form>,
            pub lines: Vec<$ref_type<$line_name>>,
            pub tracks: Vec<$ref_type<Track>>,
            pub sections: Vec<$ref_type<$section_name>>,
        }
    }
}

impl_form!(Rc, RcLine, RcSlice, RcBarLayer, RcBar, RcSection, RcTab);
impl_form!(Arc, ArcLine, ArcSlice, ArcBarLayer, ArcBar, ArcSection, ArcTab);