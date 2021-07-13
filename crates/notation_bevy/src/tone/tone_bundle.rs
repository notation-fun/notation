use bevy::prelude::*;

use notation_model::prelude::Tone;

#[derive(Bundle)]
pub struct ToneBundle {
    pub name: Name,
    pub tone: Tone,
}

impl From<Tone> for ToneBundle {
    fn from(v: Tone) -> Self {
        ToneBundle {
            name: Name::from(format!("{}", v).as_str()),
            tone: v,
        }
    }
}
