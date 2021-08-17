use std::sync::Arc;

use helgoboss_midi::{ShortMessage, StructuredShortMessage};
use notation_model::prelude::*;

#[derive(Clone, Debug)]
pub struct MidiMessage {
    pub entry: Arc<LaneEntry>,
    pub delay: Option<Units>,
    pub midi: StructuredShortMessage,
}
impl MidiMessage {
    pub fn new(entry: &Arc<LaneEntry>, delay: Option<Units>, midi: StructuredShortMessage) -> Self {
        Self {
            entry: entry.clone(),
            delay,
            midi,
        }
    }
    pub fn bar_position(&self) -> BarPosition {
        match self.delay {
            Some(delay) => self.entry.bar_position().with_delay(delay),
            None => self.entry.bar_position(),
        }
    }
    pub fn units_position(&self) -> Units {
        self.bar_position().into()
    }
    pub fn to_midi(&self) -> [u8; 3] {
        [
            self.midi.status_byte(),
            self.midi.data_byte_1().into(),
            self.midi.data_byte_2().into(),
        ]
    }
}
