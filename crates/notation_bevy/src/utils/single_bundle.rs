use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub struct SingleBundle<T: Send + Sync + 'static> {
    pub name: Name,
    pub target: T,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl<T: Send + Sync + ToString> From<T> for SingleBundle<T> {
    fn from(v: T) -> Self {
        let name = v.to_string().as_str().into();
        Self {
            name,
            target: v,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}

impl<T: Send + Sync + ToString> From<(T, Transform)> for SingleBundle<T> {
    fn from(v: (T, Transform)) -> Self {
        let name = v.0.to_string().as_str().into();
        Self {
            name,
            target: v.0,
            transform: v.1,
            global_cransform: GlobalTransform::default(),
        }
    }
}