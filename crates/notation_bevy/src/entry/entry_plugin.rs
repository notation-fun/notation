use std::sync::Arc;
use bevy::prelude::*;

use notation_proto::prelude::{CoreEntry, ProtoEntry};
use crate::prelude::{EntryBundle, AddEntryEvent, NoteBundle};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<AddEntryEvent>()
            .add_system(on_add_entry.system())
            .add_system_set(crate::note::note_systems::new_system_set())
        ;
    }
}

fn on_add_entry(mut commands: Commands,
        mut evts: EventReader<AddEntryEvent>) {
    for evt in evts.iter() {
        let line = evt.0.clone();
        let entry = evt.1.clone();
        let position = evt.2;
        let entry_bundle = EntryBundle::from((entry.clone(), position));
        let mut entry_commands = commands.spawn_bundle(entry_bundle);
        EntryPlugin::insert_extra_bundle(&mut entry_commands, entry);
        let entry_entity = entry_commands.id();
        commands.entity(line).push_children(&[entry_entity]);
    }
}

impl EntryPlugin {
    pub fn insert_extra_core_bundle(commands: &mut bevy::ecs::system::EntityCommands,
            entry: &CoreEntry) {
        match entry {
            CoreEntry::Rest(_) =>
                (),
            CoreEntry::Note(note, _) => {
                commands.insert_bundle(NoteBundle::from(*note));
                ()
            }
            CoreEntry::Solfege(solfege, _) => {
                commands.insert_bundle(NoteBundle::from(*solfege));
                ()
            }
            CoreEntry::Chord(_, _) =>
                (),
            CoreEntry::Roman(_, _) =>
                (),
            CoreEntry::Signature(_) =>
                (),
            CoreEntry::Tempo(_) =>
                (),
        };
    }

    pub fn insert_extra_bundle(commands: &mut bevy::ecs::system::EntityCommands,
            entry: Arc<ProtoEntry>) {
        match entry.as_ref() {
            ProtoEntry::Core(entry) => Self::insert_extra_core_bundle(commands, entry),
            ProtoEntry::Guitar(_) => (),
        }
    }
}
