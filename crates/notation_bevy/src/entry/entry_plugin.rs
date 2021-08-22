use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::{chord::chord_view::ChordView, prelude::{
    AddEntryEvent, BevyUtil, ChordBundle, EntryBundle, LyricsPlugin, ShapesPlugin, StringsPlugin,
    ToneBundle,
}};
use notation_model::prelude::{CoreEntry, LaneEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddEntryEvent>();
        app.add_system_set(crate::tone::tone_systems::new_system_set());
        app.add_system_set(crate::word::word_systems::new_system_set());
        app.add_system(on_add_entry.system());
        app.add_system(ChordView::on_added.system());
        app.add_system(ChordView::on_layout_changed.system());
        app.add_system(ChordView::on_bar_playing_changed.system());
    }
}

fn on_add_entry(mut commands: Commands, mut evts: EventReader<AddEntryEvent>) {
    for evt in evts.iter() {
        let parent = evt.0;
        let entry_bundle = EntryBundle::from(evt.1.clone());
        let entry_entity = BevyUtil::spawn_child_bundle(&mut commands, parent, entry_bundle);
        let mut entry_commands = commands.entity(entry_entity);
        insert_entry_extra(&mut entry_commands, &evt.1);
    }
}

fn insert_core_entry_extra(commands: &mut EntityCommands, entry: &CoreEntry) {
    match entry {
        CoreEntry::Tie => (),
        CoreEntry::Rest(_) => (),
        CoreEntry::Tone(tone, _) => {
            commands.insert_bundle(ToneBundle::from(*tone));
        }
        CoreEntry::Chord(chord, _) => {
            commands.insert_bundle(ChordBundle::from(*chord));
        }
    };
}
fn insert_entry_extra(commands: &mut EntityCommands, entry: &LaneEntry) {
    match entry.model.proto.as_ref() {
        ProtoEntry::Core(entry) => insert_core_entry_extra(commands, entry),
        ProtoEntry::Lyric(entry) => LyricsPlugin::insert_entry_extra(commands, entry),
        ProtoEntry::Fretted6(entry) => {
            ShapesPlugin::insert_entry_extra6(commands, entry);
            StringsPlugin::insert_entry_extra6(commands, entry);
        }
        ProtoEntry::Fretted4(entry) => {
            ShapesPlugin::insert_entry_extra4(commands, entry);
            StringsPlugin::insert_entry_extra4(commands, entry);
        }
        _ => {}
    }
}
