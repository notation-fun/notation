use bevy::prelude::*;

use notation_model::prelude::Tone;

#[derive(Bundle)]
pub struct ToneBundle {
    pub name: Name,
    pub tone: Tone,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl From<Tone> for ToneBundle {
    fn from(v: Tone) -> Self {
        ToneBundle {
            name: Name::from(format!("{}", v).as_str()),
            tone: v,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}
