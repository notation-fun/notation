use helgoboss_midi::{ShortMessage, StructuredShortMessage};
use notation_model::prelude::*;

#[derive(Clone, Debug)]
pub struct MidiMessage {
    pub pass_mode: EntryPassMode,
    pos: BarPosition,
    delay: Option<Units>,
    pub midi: StructuredShortMessage,
}
impl MidiMessage {
    pub fn new(pass_mode: EntryPassMode, pos: BarPosition, delay: Option<Units>, midi: StructuredShortMessage) -> Self {
        Self {
            pass_mode,
            pos,
            delay,
            midi,
        }
    }
    pub fn of_entry(entry: &LaneEntry, delay: Option<Units>, midi: StructuredShortMessage) -> Self {
        Self {
            pass_mode: entry.pass_mode(),
            pos: entry.bar_position(),
            delay,
            midi,
        }
    }
    pub fn bar_ordinal(&self) -> usize {
        self.pos.bar_ordinal
    }
    pub fn bar_position(&self) -> BarPosition {
        match self.delay {
            Some(delay) => self.pos.with_delay(delay),
            None => self.pos,
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
