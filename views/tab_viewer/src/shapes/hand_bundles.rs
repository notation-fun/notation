use crate::prelude::{SingleData, SingleBundle};
use notation_model::prelude::{HandShape4, HandShape6};

macro_rules! impl_hand_shape_bundle {
    ($type:ident, $hand_shape:ident) => {
        pub type $type = SingleBundle<SingleData<$hand_shape>>;
    };
}

impl_hand_shape_bundle!(HandShapeBundle6, HandShape6);
impl_hand_shape_bundle!(HandShapeBundle4, HandShape4);
