use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::{AddEntryEvent, EntryBundle, ShapesPlugin, StringsPlugin, ToneBundle};
use crate::word::word_bundle::WordBundle;
use notation_model::prelude::{CoreEntry, ModelEntry, ProtoEntry};

pub struct EntryPlugin;

impl Plugin for EntryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddEntryEvent>()
            .add_system_set(crate::tone::tone_systems::new_system_set())
            .add_system_set(crate::word::word_systems::new_system_set())
            .add_system(on_add_entry.system());
    }
}

fn on_add_entry(mut commands: Commands, mut evts: EventReader<AddEntryEvent>) {
    for evt in evts.iter() {
        let parent = evt.0;
        let entry = evt.1.clone();
        let entry_bundle = EntryBundle::from((entry.clone(), evt.2));
        let entry_entity = commands.spawn_bundle(entry_bundle).id();
        commands.entity(parent).push_children(&[entry_entity]);
        let mut entry_commands = commands.entity(entry_entity);
        EntryPlugin::insert_entry_extra(&mut entry_commands, entry);
    }
}

impl EntryPlugin {
    pub fn insert_core_entry_extra(commands: &mut EntityCommands, entry: &CoreEntry) {
        match entry {
            CoreEntry::Tie => (),
            CoreEntry::Rest(_) => (),
            CoreEntry::Tone(tone, _) => {
                commands.insert_bundle(ToneBundle::from(*tone));
            }
            CoreEntry::Chord(_, _) => (),
            CoreEntry::Signature(_) => (),
            CoreEntry::Tempo(_) => (),
        };
    }

    pub fn insert_entry_extra(commands: &mut EntityCommands, entry: Arc<ModelEntry>) {
        match entry.as_ref().value.as_ref() {
            ProtoEntry::Core(entry) => Self::insert_core_entry_extra(commands, entry),
            ProtoEntry::FrettedSix(entry) => {
                ShapesPlugin::insert_entry_extra(commands, entry);
                StringsPlugin::insert_entry_extra(commands, entry);
            }
            ProtoEntry::FrettedFour(entry) => {
                ShapesPlugin::insert_entry_extra(commands, entry);
                StringsPlugin::insert_entry_extra(commands, entry);
            }
            ProtoEntry::Mark(_) => {}
            ProtoEntry::Word(word, _) => {
                commands.insert_bundle(WordBundle::from(word.clone()));
            }
            ProtoEntry::Extra(_, _) => {}
        }
    }
}
