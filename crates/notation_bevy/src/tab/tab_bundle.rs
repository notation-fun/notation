use bevy::prelude::*;
use std::sync::Arc;

use notation_proto::prelude::{Tab, Units};

#[derive(Bundle)]
pub struct TabBundle {
    pub tab: Arc<Tab>,
    pub name: Name,
    pub length: Units,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<Arc<Tab>> for TabBundle {
    fn from(v: Arc<Tab>) -> Self {
        let name = v.to_string().as_str().into();
        Self {
            tab: v,
            name,
            length: Units(0.0),
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
