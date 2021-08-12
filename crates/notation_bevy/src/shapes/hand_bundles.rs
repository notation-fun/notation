use bevy::prelude::*;

use notation_model::prelude::{HandShape4, HandShape6};

macro_rules! impl_hand_shape_bundle {
    ($type:ident, $hand_shape:ident) => {
        #[derive(Bundle)]
        pub struct $type {
            pub name: Name,
            pub shape: $hand_shape,
            pub transform: Transform,
            pub global_cransform: GlobalTransform,
        }

        impl From<$hand_shape> for $type {
            fn from(v: $hand_shape) -> Self {
                Self {
                    name: v.to_string().as_str().into(),
                    shape: v,
                    transform: Transform::default(),
                    global_cransform: GlobalTransform::default(),
                }
            }
        }
    };
}

impl_hand_shape_bundle!(HandShapeBundle6, HandShape6);
impl_hand_shape_bundle!(HandShapeBundle4, HandShape4);
