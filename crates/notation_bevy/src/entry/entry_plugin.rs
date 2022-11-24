use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::ShapeOp;
use notation_model::lane_kind::LaneKind;

use crate::chord::chord_view::ChordView;
use crate::tone::tone_line::ToneLineData;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{
    BevyUtil, ChordBundle, EntryBundle, LyricsPlugin, NotationAssets, NotationAssetsStates,
    NotationSettings, NotationTheme, ShapesPlugin, StringsPlugin, ToneBundle,
};
use crate::shapes::shape_diagram::{ShapeDiagramData4, ShapeDiagramData6};
use crate::strings::pick_note::PickNoteData;
use crate::strings::single_string::SingleStringData;
use crate::tab::tab_events::TabBarsResizedEvent;
use crate::tone::tone_note::ToneNoteData;
use crate::word::word_text::WordTextData;
use notation_model::prelude::{CoreEntry, LaneEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(crate::tone::tone_systems::on_entry_playing_changed)
                .with_system(crate::word::word_systems::on_entry_playing_changed)
                .with_system(ChordView::on_layout_changed)
                .with_system(ChordView::on_chord_playing_changed)
                .with_system(on_tab_bars_resized),
        );
    }
}

pub fn create_entry(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    settings: &NotationSettings,
    lane_layout: &LaneLayoutData,
    lane_entity: Entity,
    entry: &Arc<LaneEntry>,
) {
    let entry_bundle = EntryBundle::from(entry.clone());
    let entry_entity = BevyUtil::spawn_child_bundle(commands, lane_entity, entry_bundle);
    insert_entry_extra(commands, assets, theme, settings, lane_layout, entry_entity, entry);
}

fn insert_entry_extra(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    settings: &NotationSettings,
    lane_layout: &LaneLayoutData,
    entry_entity: Entity,
    entry: &LaneEntry,
) {
    match entry.model.proto.as_ref() {
        ProtoEntry::Core(core_entry) => {
            insert_core_entry_extra(commands, assets, theme, settings, lane_layout.lane_kind, entry_entity, entry, core_entry)
        }
        ProtoEntry::Lyric(lyric_entry) => LyricsPlugin::insert_entry_extra(
            commands,
            assets,
            theme,
            settings,
            entry_entity,
            entry,
            lyric_entry,
        ),
        ProtoEntry::Fretted6(fretted_entry) => {
            ShapesPlugin::insert_entry_extra6(
                commands,
                assets,
                theme,
                settings,
                entry_entity,
                entry,
                fretted_entry,
            );
            StringsPlugin::insert_entry_extra6(
                commands,
                assets,
                theme,
                settings,
                lane_layout.lane_kind,
                entry_entity,
                entry,
                fretted_entry,
            );
        }
        ProtoEntry::Fretted4(fretted_entry) => {
            ShapesPlugin::insert_entry_extra4(
                commands,
                assets,
                theme,
                settings,
                entry_entity,
                entry,
                fretted_entry,
            );
            StringsPlugin::insert_entry_extra4(
                commands,
                assets,
                theme,
                settings,
                lane_layout.lane_kind,
                entry_entity,
                entry,
                fretted_entry,
            );
        }
        _ => {}
    }
}

fn insert_core_entry_extra(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    settings: &NotationSettings,
    lane_kind: LaneKind,
    entity: Entity,
    entry: &LaneEntry,
    core_entry: &CoreEntry,
) {
    match core_entry {
        CoreEntry::Tie => (),
        CoreEntry::Rest(_) => (),
        CoreEntry::Tone(tone, _) => {
            commands
                .entity(entity)
                .insert(ToneBundle::from(*tone));
            crate::tone::tone_systems::create_tone_notes(
                commands, assets, theme, settings, lane_kind.into(), entity, entry, tone,
            );
        }
        CoreEntry::Chord(chord, _) => {
            commands
                .entity(entity)
                .insert(ChordBundle::from(*chord));
        }
    };
}

fn on_tab_bars_resized(
    mut evts: EventReader<TabBarsResizedEvent>,
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut tone_note_query: Query<(Entity, &mut ToneNoteData), With<ToneNoteData>>,
    mut tone_line_query: Query<(Entity, &mut ToneLineData), With<ToneLineData>>,
    mut pick_note_query: Query<(Entity, &mut PickNoteData), With<PickNoteData>>,
    mut single_string_query: Query<(Entity, &mut SingleStringData), With<SingleStringData>>,
    mut word_text_query: Query<(Entity, &mut WordTextData), With<WordTextData>>,
    mut shape_diagram_6_query: Query<(Entity, &mut ShapeDiagramData6), With<ShapeDiagramData6>>,
    mut shape_diagram_4_query: Query<(Entity, &mut ShapeDiagramData4), With<ShapeDiagramData4>>,
) {
    if theme._bypass_systems {
        return;
    }
    for evt in evts.iter() {
        let bars = &evt.0;
        for (entity, mut data) in tone_note_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
        for (entity, mut data) in tone_line_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
        for (entity, mut data) in word_text_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    data.update(&mut commands, &theme, entity);
                }
            }
        }
        if !settings.hide_shapes_lane {
            for (entity, mut data) in shape_diagram_6_query.iter_mut() {
                for (view, layout) in bars.iter() {
                    if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                        data.value.bar_size = layout.size.width;
                        data.update(&mut commands, &theme, entity);
                    }
                }
            }
            for (entity, mut data) in shape_diagram_4_query.iter_mut() {
                for (view, layout) in bars.iter() {
                    if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                        data.value.bar_size = layout.size.width;
                        data.update(&mut commands, &theme, entity);
                    }
                }
            }
        }
        if !settings.hide_strings_lane {
            for (entity, mut data) in single_string_query.iter_mut() {
                for (view, layout) in bars.iter() {
                    if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                        data.value.bar_size = layout.size.width;
                        data.update(&mut commands, &theme, entity);
                    }
                }
            }
            for (entity, mut data) in pick_note_query.iter_mut() {
                for (view, layout) in bars.iter() {
                    if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                        data.value.bar_size = layout.size.width;
                        data.update(&mut commands, &theme, entity);
                    }
                }
            }
        }
    }
}
