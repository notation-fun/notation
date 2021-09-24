use midir::{MidiOutput, MidiOutputConnection};
use notation_model::prelude::{PlaySpeed, Tab};
use std::sync::{Arc, Mutex};

use crate::prelude::{MidiMessage, MidiSettings, MidiState, MidiSynth};

pub struct MidiHub {
    pub output_conn: Option<Mutex<MidiOutputConnection>>,
    pub output_synth: Option<MidiSynth>,
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
    pub fn new_output() -> Option<MidiOutput> {
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
    pub fn new_output_conn() -> Option<MidiOutputConnection> {
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
    pub fn check_output_conn(&mut self) {
        if self.output_conn.is_none() {
            self.output_conn = Self::new_output_conn().map(Mutex::new);
        }
    }
    pub fn check_output_synth(&mut self) {
        if self.output_synth.is_none() {
            self.output_synth = MidiSynth::try_new();
        }
    }
    pub fn check_output(&mut self, settings: &MidiSettings) {
        if settings.use_internal_synth {
            self.check_output_synth();
        } else {
            self.check_output_conn();
        }
    }
    pub fn switch_tab(&mut self, settings: &MidiSettings, state: &mut MidiState, tab: Arc<Tab>) {
        state.switch_tab(&settings, self, tab.clone());
        self.init_channels(settings, state);
    }
    pub fn init_channels(&mut self, settings: &MidiSettings, state: &MidiState) {
        self.check_output(settings);
        if let Some(synth) = &self.output_synth {
            synth.init_channels(settings, state);
        }
    }
    pub fn send(&mut self, settings: &MidiSettings, speed: &PlaySpeed, msg: &MidiMessage) {
        self.check_output(settings);
        if let Some(synth) = &self.output_synth {
            if let Err(err) = synth.send(speed, msg) {
                println!("send to synth failed: {:?} -> {:?}", msg, err);
            }
        }
        if let Some(conn) = &self.output_conn {
            //println!("send midi: {:?}", msg);
            if let Err(err) = conn.lock().unwrap().send(&msg.to_midi()) {
                println!("send to midi failed: {:?} -> {:?}", msg, err);
            }
        }
    }
}
