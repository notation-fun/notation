use helgoboss_midi::{ShortMessage, StructuredShortMessage};
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::Mutex;

pub struct MidiHub {
    output_conn: Option<Mutex<MidiOutputConnection>>,
}

impl Default for MidiHub {
    fn default() -> Self {
        Self { output_conn: None }
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
    pub fn send(&mut self, msg: StructuredShortMessage) {
        self.check_output_conn();
        if let Some(conn) = &self.output_conn {
            //println!("send midi: {:?}", msg);
            if let Err(err) = conn.lock().unwrap().send(&[
                msg.status_byte(),
                msg.data_byte_1().into(),
                msg.data_byte_2().into(),
            ]) {
                println!("send midi failed: {:?} -> {:?}", msg, err);
            }
        }
    }
}
