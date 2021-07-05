use bevy::prelude::*;

use notation_fretted::prelude::Pick;

#[derive(Bundle)]
pub struct PickBundle {
    pub name: Name,
    pub pick: Pick,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<Pick> for PickBundle {
    fn from(v: Pick) -> Self {
        PickBundle {
            name: v.to_string().as_str().into(),
            pick: v,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
