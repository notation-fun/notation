use std::sync::Arc;

use bevy::prelude::*;

use crate::chord::chord_view::ChordView;
use crate::prelude::{BevyUtil, ChordBundle, EntryBundle, LyricsPlugin, NotationAssets, NotationAssetsStates, NotationSettings, NotationTheme, ShapesPlugin, StringsPlugin, ToneBundle};
use notation_model::prelude::{CoreEntry, LaneEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(crate::tone::tone_systems::on_entry_playing_changed.system())
                .with_system(crate::word::word_systems::on_entry_playing_changed.system())
                .with_system(ChordView::on_added.system())
                .with_system(ChordView::on_layout_changed.system())
                .with_system(ChordView::on_chord_playing_changed.system()),
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
        ProtoEntry::Core(core_entry) => insert_core_entry_extra(commands, assets, theme, settings, entity, entry, core_entry),
        ProtoEntry::Lyric(lyric_entry) => LyricsPlugin::insert_entry_extra(commands, assets, theme, settings, entity, entry, lyric_entry),
        ProtoEntry::Fretted6(fretted_entry) => {
            ShapesPlugin::insert_entry_extra6(commands, assets, theme, settings, entity, entry, fretted_entry);
            StringsPlugin::insert_entry_extra6(commands, assets, theme, settings, entity, entry, fretted_entry);
        }
        ProtoEntry::Fretted4(fretted_entry) => {
            ShapesPlugin::insert_entry_extra4(commands, assets, theme, settings, entity, entry, fretted_entry);
            StringsPlugin::insert_entry_extra4(commands, assets, theme, settings, entity, entry, fretted_entry);
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
    core_entry: &CoreEntry
) {
    match core_entry {
        CoreEntry::Tie => (),
        CoreEntry::Rest(_) => (),
        CoreEntry::Tone(tone, _) => {
            commands.entity(entity).insert_bundle(ToneBundle::from(*tone));
            crate::tone::tone_systems::create_tone_notes(commands, assets, theme, settings, entity, entry, tone);
        }
        CoreEntry::Chord(chord, _) => {
            commands.entity(entity).insert_bundle(ChordBundle::from(*chord));
        }
    };
}
