use edger_bevy_app::bevy_prelude::*;

use edger_bevy_app::prelude::{text, ShapeOp};
use notation_model::prelude::LaneEntry;

use crate::prelude::{EntryPlaying, NotationAssets, NotationSettings, NotationTheme, ToneBundle, ToneMode};
use notation_model::prelude::Pick;

use super::pick_note::{PickNoteData, PickNoteValue};

pub fn on_entry_playing_changed(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut note_query: ParamSet<(
        Query<(Entity, &mut PickNoteData, &Children)>,
        Query<(Entity, &mut PickNoteData)>,
    )>,
    mut font_query: Query<&mut Text>,
) {
    if theme._bypass_systems {
        return;
    }
    if settings.hide_strings_lane {
        return;
    }
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data, note_children)) = note_query.p0().get_mut(*child) {
                data.value.playing_state = playing.value;
                data.update(&mut commands, &theme, entity);
                for child in note_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        text::set_color(&mut text, data.calc_fret_color(&theme));
                    }
                }
            } else if let Ok((entity, mut data)) = note_query.p1().get_mut(*child) {
                data.value.playing_state = playing.value;
                data.update(&mut commands, &theme, entity);
            }
        }
    }
}

macro_rules! impl_pick_system {
    ($create_pick_notes:ident, $create_pick_tones:ident, $fretboard:ident, $hand_shape:ident, $get_fretted_shape:ident
    ) => {
        pub fn $create_pick_notes(
            commands: &mut Commands,
            assets: &NotationAssets,
            theme: &NotationTheme,
            settings: &NotationSettings,
            entity: Entity,
            entry: &LaneEntry,
            pick: &Pick,
        ) {
            /* TODO: check whether is the first bar in row
            if entry.as_ref().prev_is_tie() {
                continue;
            }
            */
            if let Some(bar) = entry.bar() {
                if let Some((fretboard, shape)) = bar.$get_fretted_shape(entry) {
                    let meta = bar.tab_meta();
                    for pick_note in pick.get_notes() {
                        if let Some((fret, note)) =
                            fretboard.shape_pick_fret_note(&meta.scale, &meta.key, &shape, pick_note)
                        {
                            let syllable = bar.calc_syllable(&note.pitch);
                            let data =
                                PickNoteData::new(entry, PickNoteValue::new(pick_note, syllable));
                            let note_entity = data.create(commands, theme, entity);
                            if !settings.hide_strings_lane
                                && (settings.always_show_fret || pick_note.fret.is_some())
                            {
                                theme.texts.strings.spawn_fret_text(
                                    commands,
                                    note_entity,
                                    &assets,
                                    fret,
                                );
                            }
                        }
                    }
                }
            }
        }
        pub fn $create_pick_tones(
            commands: &mut Commands,
            assets: &NotationAssets,
            theme: &NotationTheme,
            settings: &NotationSettings,
            entity: Entity,
            entry: &LaneEntry,
            pick: &Pick,
        ) {
            /* TODO: check whether is the first bar in row
            if entry.as_ref().prev_is_tie() {
                continue;
            }
            */
            if let Some(bar) = entry.bar() {
                if let Some((fretboard, shape)) = bar.$get_fretted_shape(entry) {
                    let meta = bar.tab_meta();
                    let tone = fretboard.pick_tone(&meta.scale, &meta.key, &shape, pick);
                    commands
                        .entity(entity)
                        .insert(ToneBundle::from(tone));
                    crate::tone::tone_systems::create_tone_notes(
                        commands, assets, theme, settings, ToneMode::Harmony, entity, entry, &tone,
                    );
                }
            }
        }
    };
}

impl_pick_system!(
    create_pick_notes6,
    create_pick_tones6,
    Fretboard6,
    HandShape6,
    get_fretted_shape6
);
impl_pick_system!(
    create_pick_notes4,
    create_pick_tones4,
    Fretboard4,
    HandShape4,
    get_fretted_shape4
);
