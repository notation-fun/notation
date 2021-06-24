use bevy::prelude::*;

use notation_proto::prelude::*;

#[derive(Clone)]
pub struct AddLineEvent(pub ArcLine);