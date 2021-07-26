use crate::prelude::{MidiHub, MidiUtil, PlayToneEvent, StopToneEvent};
use bevy::prelude::*;

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlayToneEvent>();
        app.add_event::<StopToneEvent>();
        app.init_non_send_resource::<MidiHub>();
        app.add_system(on_play_tone.system());
        app.add_system(on_stop_tone.system());
    }
}

fn on_play_tone(
    mut _commands: Commands,
    mut evts: EventReader<PlayToneEvent>,
    mut hub: NonSendMut<MidiHub>,
) {
    for evt in evts.iter() {
        //println!("on_play_tone: {}", evt.0);
        for msg in MidiUtil::tone_midi_on_msgs(&evt.0) {
            hub.send(msg);
        }
    }
}

fn on_stop_tone(
    mut _commands: Commands,
    mut evts: EventReader<StopToneEvent>,
    mut hub: NonSendMut<MidiHub>,
) {
    for evt in evts.iter() {
        //println!("on_stop_tone: {}", evt.0);
        for msg in MidiUtil::tone_midi_off_msgs(&evt.0) {
            hub.send(msg);
        }
    }
}
