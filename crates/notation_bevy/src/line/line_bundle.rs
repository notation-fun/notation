use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{Line, Units};

#[derive(Bundle)]
pub struct LineBundle {
    pub line: Arc<Line>,
    pub name: Name,
    pub length: Units,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<Arc<Line>> for LineBundle {
    fn from(v: Arc<Line>) -> Self {
        let name = Name::from(v.name.clone().as_str());
        Self {
            line: v,
            name,
            length: Units(0.0),
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
