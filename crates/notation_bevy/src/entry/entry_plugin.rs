use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::LyonShapeOp;

use crate::chord::chord_view::ChordView;
use crate::prelude::{
    BevyUtil, ChordBundle, EntryBundle, LyricsPlugin, NotationAssets, NotationAssetsStates,
    NotationSettings, NotationTheme, ShapesPlugin, StringsPlugin, ToneBundle,
};
use crate::shapes::shape_diagram::{
    ShapeDiagram4, ShapeDiagram6, ShapeDiagramData4, ShapeDiagramData6,
};
use crate::strings::pick_note::{PickNoteData, PickNoteShape};
use crate::strings::single_string::{SingleString, SingleStringData};
use crate::tab::tab_events::TabBarsResizedEvent;
use crate::tone::tone_note::{ToneNoteData, ToneNoteShape};
use crate::word::word_text::{WordTextData, WordTextShape};
use notation_model::prelude::{CoreEntry, LaneEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(crate::tone::tone_systems::on_entry_playing_changed.system())
                .with_system(crate::word::word_systems::on_entry_playing_changed.system())
                .with_system(ChordView::on_layout_changed.system())
                .with_system(ChordView::on_chord_playing_changed.system())
                .with_system(on_tab_bars_resized.system()),
        );
    }
}

pub fn create_entry(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    settings: &NotationSettings,
    entity: Entity,
    entry: &Arc<LaneEntry>,
) {
    let entry_bundle = EntryBundle::from(entry.clone());
    let entry_entity = BevyUtil::spawn_child_bundle(commands, entity, entry_bundle);
    insert_entry_extra(commands, assets, theme, settings, entry_entity, entry);
}

fn insert_entry_extra(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    settings: &NotationSettings,
    entity: Entity,
    entry: &LaneEntry,
) {
    match entry.model.proto.as_ref() {
        ProtoEntry::Core(core_entry) => {
            insert_core_entry_extra(commands, assets, theme, settings, entity, entry, core_entry)
        }
        ProtoEntry::Lyric(lyric_entry) => LyricsPlugin::insert_entry_extra(
            commands,
            assets,
            theme,
            settings,
            entity,
            entry,
            lyric_entry,
        ),
        ProtoEntry::Fretted6(fretted_entry) => {
            ShapesPlugin::insert_entry_extra6(
                commands,
                assets,
                theme,
                settings,
                entity,
                entry,
                fretted_entry,
            );
            StringsPlugin::insert_entry_extra6(
                commands,
                assets,
                theme,
                settings,
                entity,
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
                entity,
                entry,
                fretted_entry,
            );
            StringsPlugin::insert_entry_extra4(
                commands,
                assets,
                theme,
                settings,
                entity,
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
                .insert_bundle(ToneBundle::from(*tone));
            crate::tone::tone_systems::create_tone_notes(
                commands, assets, theme, settings, entity, entry, tone,
            );
        }
        CoreEntry::Chord(chord, _) => {
            commands
                .entity(entity)
                .insert_bundle(ChordBundle::from(*chord));
        }
    };
}

fn on_tab_bars_resized(
    mut evts: EventReader<TabBarsResizedEvent>,
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut tone_note_query: Query<(Entity, &mut ToneNoteData), With<ToneNoteData>>,
    mut pick_note_query: Query<(Entity, &mut PickNoteData), With<PickNoteData>>,
    mut single_string_query: Query<(Entity, &mut SingleStringData), With<SingleStringData>>,
    mut word_text_query: Query<(Entity, &mut WordTextData), With<WordTextData>>,
    mut shape_diagram_6_query: Query<(Entity, &mut ShapeDiagramData6), With<ShapeDiagramData6>>,
    mut shape_diagram_4_query: Query<(Entity, &mut ShapeDiagramData4), With<ShapeDiagramData4>>,
) {
    for evt in evts.iter() {
        let bars = &evt.0;
        for (entity, mut data) in tone_note_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    ToneNoteShape::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in pick_note_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    PickNoteShape::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in single_string_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    SingleString::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in word_text_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    WordTextShape::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in shape_diagram_6_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    ShapeDiagram6::update(&mut commands, &theme, entity, &data);
                }
            }
        }
        for (entity, mut data) in shape_diagram_4_query.iter_mut() {
            for (view, layout) in bars.iter() {
                if data.bar_props.bar_ordinal == view.bar_props.bar_ordinal {
                    data.value.bar_size = layout.size.width;
                    ShapeDiagram4::update(&mut commands, &theme, entity, &data);
                }
            }
        }
    }
}
