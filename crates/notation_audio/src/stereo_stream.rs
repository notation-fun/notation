use bevy::prelude::*;
use bevy_kira_audio::{AudioStream, Frame, StreamedAudio, AudioPlugin, AudioStreamPlugin};
use ringbuf::{RingBuffer, Consumer, Producer};

pub struct StereoStreamOutput {
    buffer: Consumer<(f32, f32)>,
}

impl std::fmt::Debug for StereoStreamOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StereoStreamOutput>({}/{})", self.buffer.len(), self.buffer.capacity())
    }
}

impl AudioStream for StereoStreamOutput {
    fn next(&mut self, _: f64) -> Frame {
        if self.buffer.is_full() {
            let capacity = self.buffer.capacity();
            let dropped = self.buffer.discard(self.buffer.capacity() / 2);
            println!("<StereoStreamOutput>[{}] buffer is full, dropped: {}", capacity, dropped);
        }
        let data = self.buffer.pop().unwrap_or((0.0, 0.0));
        Frame::new(data.0, data.1)
    }
}

impl Default for StereoStreamOutput {
    fn default() -> Self {
        let buffer = RingBuffer::new(1024);
        let (_producer, consumer) = buffer.split();
        Self { buffer: consumer }
    }
}

pub struct StereoStream {
    pub buffer: Producer<(f32, f32)>,
    pub volume: f32,
}

impl std::fmt::Debug for StereoStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StereoStream>({}/{}, v:{})", self.buffer.len(), self.buffer.capacity(), self.volume)
    }
}

impl StereoStream {
    pub const FRAME_STEP: f64 = 1.0 / 44_000.0;

    pub const DEFAULT_CAPACITY: usize = 4096;
    pub const DEFAULT_VOLUME: f32 = 1.0;
    pub fn push(&mut self, left: f32, right: f32) {
        if let Err(err) = self.buffer.push((left * self.volume, right * self.volume)) {
            println!("<StereoStream> push failed: {}, {} -> {:?}", left, right, err);
        }
    }
    pub fn init_streaming(
        app: &mut AppBuilder,
        setup_default_streaming: bool,
    ) {
        app.add_plugin(AudioPlugin);
        app.add_plugin(AudioStreamPlugin::<StereoStreamOutput>::default());
        if setup_default_streaming {
            app.add_startup_system(Self::setup_default_streaming.system());
        }
    }
    pub fn setup_streaming(
        commands: &mut Commands,
        streamed_audio: &StreamedAudio<StereoStreamOutput>,
        capacity: usize,
        volume: f32,
    ) {
        let buffer = RingBuffer::<(f32, f32)>::new(capacity);
        let (producer, consumer) = buffer.split();
        let stream = StereoStreamOutput { buffer: consumer };
        streamed_audio.stream(stream);
        let buffer = StereoStream { buffer: producer, volume };
        commands.insert_resource(buffer);
    }
    pub fn setup_default_streaming(
        mut commands: Commands,
        streamed_audio: Res<StreamedAudio<StereoStreamOutput>>,
    ) {
        Self::setup_streaming(&mut commands, &streamed_audio, Self::DEFAULT_CAPACITY, Self::DEFAULT_VOLUME);
    }
}