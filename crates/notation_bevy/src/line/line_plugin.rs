use bevy::prelude::*;

use crate::prelude::{AddEntryEvent, AddLineEvent};
use notation_proto::prelude::Units;

use super::line_bundle::LineBundle;

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddLineEvent>()
            .add_system(on_add_line.system());
    }
}

fn on_add_line(
    mut commands: Commands,
    mut set_units: Query<&mut Units>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
    mut evts: EventReader<AddLineEvent>,
) {
    for evt in evts.iter() {
        let line = evt.0.clone();
        let line_entity = commands.spawn_bundle(LineBundle::from(line.clone())).id();
        let mut position = Units(0.0);
        for entry in line.entries.iter() {
            let duration = entry.duration();
            add_entry_evts.send(AddEntryEvent(line_entity, entry.clone(), position));
            position = position + Units::from(duration);
        }
        if let Ok(mut units) = set_units.get_mut(line_entity) {
            *units = position;
        }
    }
}
