use helgoboss_midi::{ShortMessage, StructuredShortMessage};
use notation_model::prelude::*;

#[derive(Clone, Debug)]
pub struct MidiMessage {
    pub pass_mode: EntryPassMode,
    pub pos: BarPosition,
    pub duration: Units,
    pub delay: bool,
    pub midi: StructuredShortMessage,
}
impl MidiMessage {
    pub const DELAY_GAP: Units = Units(Units::_MIN_ACCURACY * 2.0);
    pub fn new(
        pass_mode: EntryPassMode,
        pos: BarPosition,
        duration: Units,
        delay: bool,
        midi: StructuredShortMessage,
    ) -> Self {
        Self {
            pass_mode,
            pos,
            duration,
            delay,
            midi,
        }
    }
    pub fn of_entry(entry: &LaneEntry, delay: bool, midi: StructuredShortMessage) -> Self {
        Self {
            pass_mode: entry.pass_mode(),
            pos: entry.bar_position(),
            duration: entry.tied_units(),
            delay,
            midi,
        }
    }
    pub fn bar_ordinal(&self) -> usize {
        self.pos.bar_ordinal
    }
    pub fn effect_position(&self) -> BarPosition {
        if self.delay {
            self.pos.with_delay(self.duration - Self::DELAY_GAP)
        } else {
            self.pos
        }
    }
    pub fn effect_units(&self) -> Units {
        self.effect_position().into()
    }
    pub fn to_midi(&self) -> [u8; 3] {
        [
            self.midi.status_byte(),
            self.midi.data_byte_1().into(),
            self.midi.data_byte_2().into(),
        ]
    }
}
