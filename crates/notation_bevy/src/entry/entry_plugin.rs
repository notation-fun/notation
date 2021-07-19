use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::{AddEntryEvent, EntryBundle, FrettedPlugin, ToneBundle};
use notation_model::prelude::{CoreEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddEntryEvent>()
            .add_system(on_add_entry.system())
            .add_system_set(crate::tone::tone_systems::new_system_set());
    }
}

fn on_add_entry(mut commands: Commands, mut evts: EventReader<AddEntryEvent>) {
    for evt in evts.iter() {
        let parent = evt.0;
        let entry = evt.1.clone();
        let entry_bundle = EntryBundle::from((entry.clone(), evt.2));
        let mut entry_commands = commands.spawn_bundle(entry_bundle);
        EntryPlugin::insert_entry_extra(&mut entry_commands, entry);
        let entry_entity = entry_commands.id();
        commands.entity(parent).push_children(&[entry_entity]);
    }
}

impl EntryPlugin {
    pub fn insert_core_entry_extra(commands: &mut EntityCommands, entry: &CoreEntry) {
        match entry {
            CoreEntry::Rest(_) => (),
            CoreEntry::Tone(tone, _) => {
                commands.insert_bundle(ToneBundle::from(*tone));
            }
            CoreEntry::Chord(_, _) => (),
            CoreEntry::Signature(_) => (),
            CoreEntry::Tempo(_) => (),
        };
    }

    pub fn insert_entry_extra(commands: &mut EntityCommands, entry: Arc<ProtoEntry>) {
        match entry.as_ref() {
            ProtoEntry::Core(entry) => Self::insert_core_entry_extra(commands, entry),
            ProtoEntry::FrettedSix(entry) => {
                FrettedPlugin::insert_fretted_entry_extra(commands, entry);
            }
            ProtoEntry::FrettedFour(entry) => {
                FrettedPlugin::insert_fretted_entry_extra(commands, entry);
            }
            ProtoEntry::Mark(_) => {}
            ProtoEntry::Extra(_, _) => {}
        }
    }
}
