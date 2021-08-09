use std::sync::{Arc, Mutex};

use bevy_kira_audio::{AudioStream, Frame};

pub const AUDIO_BUFFER_SIZE: usize = 2048 * 2;

pub type AudioBuffer = [f32; AUDIO_BUFFER_SIZE];

#[derive(Clone, Debug)]
pub struct DoubleAudioBuffer {
    pub use_buffer_2: bool,
    pub buffer_1: Arc<Mutex<(bool, AudioBuffer)>>,
    pub buffer_2: Arc<Mutex<(bool, AudioBuffer)>>,
}
impl DoubleAudioBuffer {
    pub fn new() -> Self {
        Self {
            use_buffer_2: false,
            buffer_1: Arc::new(Mutex::new((false, [0f32; AUDIO_BUFFER_SIZE]))),
            buffer_2: Arc::new(Mutex::new((false, [0f32; AUDIO_BUFFER_SIZE]))),
        }
    }
    pub fn write_buffer<F>(&mut self, mut action: F) where F: FnMut(&mut [f32]) {
        let buffer = if self.use_buffer_2 {
            self.buffer_2.as_ref()
        } else {
            self.buffer_1.as_ref()
        };
        if let Ok(mut val) = buffer.lock() {
            if !val.0 {
                action(val.1.as_mut());
                val.0 = true;
                self.use_buffer_2 = !self.use_buffer_2;
            }
        }
    }
    pub fn read_buffer<F>(&mut self, mut action: F) where F: FnMut(&AudioBuffer) {
        let buffer = if self.use_buffer_2 {
            self.buffer_2.as_ref()
        } else {
            self.buffer_1.as_ref()
        };
        if let Ok(mut val) = buffer.lock() {
            if val.0 {
                action(&val.1);
                val.0 = false;
                self.use_buffer_2 = !self.use_buffer_2;
            }
        }
    }
}

#[derive(Debug)]
pub struct MidiAudioStream {
    index: usize,
    buffer: AudioBuffer,
    synth_buffer: DoubleAudioBuffer,
}
impl MidiAudioStream {
    pub fn new(synth_buffer: DoubleAudioBuffer) -> Self {
        let buffer = [0f32; AUDIO_BUFFER_SIZE];
        Self {
            index: 0,
            buffer,
            synth_buffer,
        }
    }
}
impl Default for MidiAudioStream {
    fn default() -> Self {
        Self::new(DoubleAudioBuffer::new())
    }
}
impl MidiAudioStream {
    fn check_synth_buffer(&mut self) {
        let buffer = &mut self.buffer;
        let index = &mut self.index;
        self.synth_buffer.read_buffer(|data|{
            for i in 0..AUDIO_BUFFER_SIZE {
                buffer[i] = data[i];
            }
            *index = 0;
        });
    }
}
impl AudioStream for MidiAudioStream {
    fn next(&mut self, _: f64) -> Frame {
        if self.index + 1 >= AUDIO_BUFFER_SIZE {
            self.check_synth_buffer();
        }
        if self.index + 1 >= AUDIO_BUFFER_SIZE {
            return Frame::new(0.0, 0.0);
        }
        let left = self.buffer[self.index];
        let right = self.buffer[self.index + 1];
        self.index += 2;
        Frame::new(left, right)
    }
}