use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{Entry, LaneEntry, PlayingState, Position};

use crate::prelude::{EntryData, TabState};

pub type EntryPlaying = EntryData<PlayingState>;

impl EntryPlaying {
    pub fn update(
        query: &mut Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying), With<EntryPlaying>>,
        tab_state: &TabState,
    ) {
        for (_entity, entry, mut entry_playing) in query.iter_mut() {
            if tab_state.play_control.play_state.is_stopped() {
                if tab_state.is_bar_in_range(entry_playing.bar_props.bar_ordinal) {
                    if entry.bar_props().bar_ordinal == tab_state.play_control.position.bar.bar_ordinal && entry.props.in_bar_pos.0 == 0.0 {
                        entry_playing.value = PlayingState::Current;
                    } else {
                        entry_playing.value = PlayingState::Idle;
                    }
                }
            }
        }
    }
    pub fn update_with_pos(
        query: &mut Query<(Entity, &Arc<LaneEntry>, &mut EntryPlaying), With<EntryPlaying>>,
        tab_state: &TabState,
        new_position: &Position,
        end_passed: bool,
        jumped: bool,
    ) {
        let playing_bar_ordinal = new_position.bar.bar_ordinal;
        if end_passed || jumped {
            for (_entity, _entry, mut entry_playing) in query.iter_mut() {
                if entry_playing.value != PlayingState::Idle {
                    entry_playing.value = PlayingState::Idle;
                }
            }
        }
        for (_entity, entry, mut entry_playing) in query.iter_mut() {
            let bar_ordinal = entry_playing.bar_props.bar_ordinal;
            if tab_state.is_bar_in_range(bar_ordinal) {
                if entry_playing.value.is_current()
                    && new_position.is_passed_with(
                        entry.pass_mode(),
                        &entry_playing.bar_position(),
                        entry.tied_units(),
                    )
                {
                    if entry_playing.value != PlayingState::Played {
                        entry_playing.value = PlayingState::Played;
                    }
                }
                if bar_ordinal == playing_bar_ordinal
                    && entry_playing.value.is_idle()
                    && new_position.is_passed(entry.pass_mode(), &entry_playing.bar_position())
                {
                    if entry_playing.value != PlayingState::Current {
                        entry_playing.value = PlayingState::Current;
                    }
                }
            }
        }
    }
}
