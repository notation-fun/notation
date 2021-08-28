use std::sync::Arc;

use bevy::prelude::*;

use crate::prelude::BevyUtil;

#[derive(Bundle, Debug)]
pub struct SingleBundle<T: Send + Sync + 'static> {
    pub name: Name,
    pub value: T,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl<T: Send + Sync + 'static> From<(String, T, Transform)> for SingleBundle<T> {
    fn from(v: (String, T, Transform)) -> Self {
        Self {
            name: BevyUtil::calc_name(v.0),
            value: v.1,
            transform: v.2,
            global_cransform: GlobalTransform::default(),
        }
    }
}
impl<T: Send + Sync + ToString> From<(T, Transform)> for SingleBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        (v.0.to_string(), v.0, v.1).into()
    }
}
impl<T: Send + Sync + ToString> From<T> for SingleBundle<T> {
    fn from(v: T) -> Self {
        (v, Transform::default()).into()
    }
}

pub type SingleArcBundle<T> = SingleBundle<Arc<T>>;

impl<T: Send + Sync + ToString> From<(T, Transform)> for SingleArcBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        (v.0.to_string(), Arc::new(v.0), v.1).into()
    }
}
impl<T: Send + Sync + ToString> From<T> for SingleArcBundle<T> {
    fn from(v: T) -> Self {
        (v, Transform::default()).into()
    }
}
