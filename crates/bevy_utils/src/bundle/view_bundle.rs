use std::sync::Arc;

use bevy::prelude::*;

use crate::prelude::{BevyUtil, LayoutData};

#[derive(Bundle, Debug)]
pub struct ViewBundle<T: Send + Sync + 'static> {
    pub name: Name,
    pub view: Arc<T>,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl<T: Send + Sync + 'static> From<(String, T, Transform)> for ViewBundle<T> {
    fn from(v: (String, T, Transform)) -> Self {
        Self {
            name: BevyUtil::calc_name(v.0),
            view: Arc::new(v.1),
            layout: LayoutData::default(),
            transform: v.2,
            global_cransform: GlobalTransform::default(),
        }
    }
}
impl<T: Send + Sync + ToString> From<(T, Transform)> for ViewBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        (v.0.to_string(), v.0, v.1).into()
    }
}
impl<T: Send + Sync + ToString> From<T> for ViewBundle<T> {
    fn from(v: T) -> Self {
        (v, Transform::default()).into()
    }
}
