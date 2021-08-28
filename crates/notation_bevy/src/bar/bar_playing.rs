use bevy::prelude::*;

use notation_model::prelude::PlayingState;

use crate::prelude::{BarData, TabState};

pub type BarPlaying = BarData<PlayingState>;

impl BarPlaying {
    pub fn update(
        query: &mut Query<(Entity, &mut BarPlaying), With<BarPlaying>>,
        tab_state: &TabState,
        playing_bar_ordinal: usize,
    ) {
        for (_entity, mut bar_playing) in query.iter_mut() {
            let bar_ordinal = bar_playing.bar_props.bar_ordinal;
            if tab_state.is_bar_in_range(bar_ordinal) {
                if bar_ordinal == playing_bar_ordinal {
                    if bar_playing.value != PlayingState::Current {
                        bar_playing.value = PlayingState::Current;
                    }
                } else if bar_ordinal < playing_bar_ordinal {
                    if bar_playing.value != PlayingState::Played {
                        bar_playing.value = PlayingState::Played;
                    }
                } else {
                    if bar_playing.value != PlayingState::Idle {
                        bar_playing.value = PlayingState::Idle;
                    }
                }
            }
        }
    }
}
