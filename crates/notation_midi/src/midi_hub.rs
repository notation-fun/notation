use helgoboss_midi::{ShortMessage, StructuredShortMessage};
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Mutex};

use crate::prelude::{DoubleAudioBuffer, MidiSettings, MidiState, MidiSynth};

pub struct MidiHub {
    output_conn: Option<Mutex<MidiOutputConnection>>,
    output_synth: Option<MidiSynth>,
}

impl Default for MidiHub {
    fn default() -> Self {
        Self {
            output_conn: None,
            output_synth: None,
        }
    }
}

impl MidiHub {
    fn new_output() -> Option<MidiOutput> {
        if let Ok(output) = MidiOutput::new("MidiHub") {
            let ports = output.ports();
            println!("MidiHub::new_output() ports: [{}]", ports.len());
            for port in ports {
                println!("MidiHub::new_output() port: {:?}", output.port_name(&port));
            }
            Some(output)
        } else {
            None
        }
    }
    fn new_output_conn() -> Option<MidiOutputConnection> {
        if let Some(output) = Self::new_output() {
            if output.port_count() > 0 {
                #[cfg(target_os = "linux")]
                let port = &output.ports()[if output.port_count() > 1 { 1 } else { 0 }]; //TODO: Select port
                #[cfg(not(target_os = "linux"))]
                let port = &output.ports()[0]; //TODO: Select port
                output.connect(port, "MidiHub").ok()
            } else {
                None
            }
        } else {
            None
        }
    }
    fn check_output_conn(&mut self) {
        if self.output_conn.is_none() {
            self.output_conn = Self::new_output_conn().map(Mutex::new);
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn check_output_synth(&mut self) {}
    #[cfg(not(target_arch = "wasm32"))]
    fn check_output_synth(&mut self) {
        if self.output_synth.is_none() {
            self.output_synth = MidiSynth::try_new();
        }
    }
    fn check_output(&mut self, settings: &MidiSettings) {
        if settings.use_internal_synth {
            self.check_output_synth();
        } else {
            self.check_output_conn();
        }
    }
    pub fn get_synth_buffer(&mut self, settings: &MidiSettings) -> Option<DoubleAudioBuffer> {
        self.check_output(settings);
        if let Some(synth) = &self.output_synth {
            synth.get_buffer()
        } else {
            None
        }
    }
    pub fn check_synth_buffer(&mut self) {
        if let Some(synth) = self.output_synth.as_mut() {
            synth.check_buffer();
        }
    }
    pub fn setup_channels(&mut self, settings: &MidiSettings, state: &MidiState) {
        for channel in &state.channels {
            if channel.track.is_some() {
                let msg = StructuredShortMessage::ProgramChange {
                    channel: channel.channel,
                    program_number: channel.program,
                };
                self.send(settings, msg);
            }
        }
    }
    pub fn send(&mut self, settings: &MidiSettings, msg: StructuredShortMessage) {
        self.check_output(settings);
        if let Some(synth) = &self.output_synth {
            if let Err(err) = synth.send(msg) {
                println!("send to synth failed: {:?} -> {:?}", msg, err);
            }
        }
        if let Some(conn) = &self.output_conn {
            //println!("send midi: {:?}", msg);
            if let Err(err) = conn.lock().unwrap().send(&[
                msg.status_byte(),
                msg.data_byte_1().into(),
                msg.data_byte_2().into(),
            ]) {
                println!("send to midi failed: {:?} -> {:?}", msg, err);
            }
        }
    }
}
