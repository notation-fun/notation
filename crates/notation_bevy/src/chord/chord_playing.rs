use bevy::prelude::*;

use notation_model::prelude::{Chord, ModelEntryProps, PlayingState, Position};

use crate::prelude::{ModelEntryData, TabState};

pub struct ChordPlayingValue {
    pub chord: Chord,
    pub state: PlayingState,
}

pub type ChordPlaying = ModelEntryData<ChordPlayingValue>;

impl From<(ModelEntryProps, Chord)> for ChordPlaying {
    fn from(v: (ModelEntryProps, Chord)) -> Self {
        (v.0, ChordPlayingValue {
            chord: v.1,
            state: PlayingState::Idle,
        })
            .into()
    }
}

impl ChordPlaying {
    pub fn update(
        query: &mut Query<(Entity, &mut ChordPlaying), With<ChordPlaying>>,
        tab_state: &TabState,
        new_position: &Position,
    ) {
        let is_current = |c: Chord| {
            if tab_state.play_control.play_state.is_playing() {
                let chord = tab_state
                    .tab
                    .get_bar(new_position.bar)
                    .and_then(|x| x.get_chord(Some(new_position.bar.in_bar_pos)));
                if let Some(chord) = chord {
                    chord == c
                } else {
                    false
                }
            } else {
                let chords = tab_state.tab.get_bar(new_position.bar).map(|x| x.get_chords());
                if let Some(chords) = chords {
                    chords.contains(&c)
                } else {
                    false
                }
            }
        };
        for (_entity, mut chord_playing) in query.iter_mut() {
            if is_current(chord_playing.value.chord) {
                if chord_playing.value.state != PlayingState::Current {
                    chord_playing.value.state = PlayingState::Current;
                }
            } else {
                if chord_playing.value.state != PlayingState::Idle {
                    chord_playing.value.state = PlayingState::Idle;
                }
            }
        }
    }
}
