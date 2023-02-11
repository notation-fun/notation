use bevy::prelude::*;
use notation_bevy_utils::prelude::ColorBackground;
use notation_model::prelude::Chord;

use crate::prelude::NotationTheme;

#[derive(Clone, Debug, Component)]
pub struct ChordColorBackground;

impl ChordColorBackground {
    pub fn spawn(commands: &mut Commands, entity: Entity, z: f32, color: Color) -> Entity {
        let result = ColorBackground::spawn(commands, entity, z, color);
        commands.entity(result).insert(ChordColorBackground);
        result
    }
    pub fn update_color(
        commands: &mut Commands,
        theme: &NotationTheme,
        query: &mut Query<(Entity, &mut ColorBackground), With<ChordColorBackground>>,
        chord: Option<Chord>,
    ) {
        let color = theme.colors.of_option_chord(chord);
        for (entity, mut background) in query.iter_mut() {
            background.update_color(commands, entity, color);
        }
    }
}
