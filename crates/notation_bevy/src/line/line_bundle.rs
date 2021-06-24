use std::sync::Arc;

use bevy::prelude::*;

use notation_proto::prelude::{Units, ArcLine};

#[derive(Bundle)]
pub struct LineBundle {
    pub line: Arc<ArcLine>,
    pub length: Units,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<ArcLine> for LineBundle {
    fn from(v: ArcLine) -> Self {
        Self {
            line: Arc::new(v),
            length: Units(0.0),
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}