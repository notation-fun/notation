use bevy::prelude::*;

pub struct WordText(pub String);

#[derive(Bundle)]
pub struct WordBundle {
    pub name: Name,
    pub text: WordText,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<String> for WordBundle {
    fn from(v: String) -> Self {
        WordBundle {
            name: Name::from(v.as_str()),
            text: WordText(v),
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
