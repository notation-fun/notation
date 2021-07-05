use bevy::prelude::*;

use notation_fretted::prelude::HandShape;

#[derive(Bundle)]
pub struct HandShapeBundle<const S: usize> {
    pub name: Name,
    pub shape: HandShape<S>,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl<const S: usize> From<HandShape<S>> for HandShapeBundle<S> {
    fn from(v: HandShape<S>) -> Self {
        Self {
            name: v.to_string().as_str().into(),
            shape: v,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
