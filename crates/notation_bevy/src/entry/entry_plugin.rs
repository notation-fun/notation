use std::sync::Arc;
use bevy::prelude::*;

use notation_proto::prelude::{Units, Entry};
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
    pub fn insert_extra_bundle(commands: &mut bevy::ecs::system::EntityCommands,
            entry: Arc<Entry>) {
        match entry.as_ref() {
            Entry::Rest(_) =>
                (),
            Entry::Note(note, _) => {
                commands.insert_bundle(NoteBundle::from(*note));
                ()
            }
            Entry::Solfege(solfege, _) => {
                commands.insert_bundle(NoteBundle::from(*solfege));
                ()
            }
            Entry::Chord(_, _) =>
                (),
            Entry::Roman(_, _) =>
                (),
            Entry::Signature(_) =>
                (),
            Entry::Tempo(_) =>
                (),
        };
    }
}
