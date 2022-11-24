use bevy::prelude::*;
use bevy_kira_audio::{AudioStream, Frame, StreamedAudio, AudioPlugin, AudioStreamPlugin};
use ringbuf::{RingBuffer, Consumer, Producer};

pub struct MonoStreamOutput {
    buffer: Consumer<f32>,
}

impl std::fmt::Debug for MonoStreamOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<MonoStreamOutput>({}/{})", self.buffer.len(), self.buffer.capacity())
    }
}

impl AudioStream for MonoStreamOutput {
    fn next(&mut self, _: f64) -> Frame {
        if self.buffer.is_full() {
            let capacity = self.buffer.capacity();
            let dropped = self.buffer.discard(self.buffer.capacity() / 2);
            println!("<MonoStreamOutput>[{}] buffer is full, dropped: {}", capacity, dropped);
        }
        let data = self.buffer.pop().unwrap_or(0.0);
        Frame::from_mono(data)
    }
}

impl Default for MonoStreamOutput {
    fn default() -> Self {
        let buffer = RingBuffer::new(1024);
        let (_producer, consumer) = buffer.split();
        Self { buffer: consumer }
    }
}

#[derive(Resource)]
pub struct MonoStream {
    pub buffer: Producer<f32>,
    pub volume: f32,
}

impl std::fmt::Debug for MonoStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<MonoStream>({}/{}, v:{})", self.buffer.len(), self.buffer.capacity(), self.volume)
    }
}

impl MonoStream {
    pub const FRAME_STEP: f64 = 1.0 / 44_000.0;

    pub const DEFAULT_CAPACITY: usize = 4096;
    pub const DEFAULT_VOLUME: f32 = 1.0;
    pub fn push(&mut self, data: f32) {
        if let Err(err) = self.buffer.push(data * self.volume) {
            println!("<MonoStream> push failed: {} -> {:?}", data, err);
        }
    }
    pub fn init_streaming(
        app: &mut App,
        setup_default_streaming: bool,
    ) {
        app.add_plugin(AudioPlugin);
        app.add_plugin(AudioStreamPlugin::<MonoStreamOutput>::default());
        if setup_default_streaming {
            app.add_startup_system(Self::setup_default_streaming);
        }
    }
    pub fn setup_streaming(
        commands: &mut Commands,
        streamed_audio: &StreamedAudio<MonoStreamOutput>,
        capacity: usize,
        volume: f32,
    ) {
        let buffer = RingBuffer::<f32>::new(capacity);
        let (producer, consumer) = buffer.split();
        let stream = MonoStreamOutput { buffer: consumer };
        streamed_audio.stream(stream);
        let buffer = MonoStream { buffer: producer, volume };
        commands.insert_resource(buffer);
    }
    pub fn setup_default_streaming(
        mut commands: Commands,
        streamed_audio: Res<StreamedAudio<MonoStreamOutput>>,
    ) {
        Self::setup_streaming(&mut commands, &streamed_audio, Self::DEFAULT_CAPACITY, Self::DEFAULT_VOLUME);
    }
}