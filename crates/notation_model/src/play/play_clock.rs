#[cfg(target_arch = "wasm32")]
use instant::Duration as StdDuration;
#[cfg(target_arch = "wasm32")]
use instant::Instant as StdInstant;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration as StdDuration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;

pub struct PlayClock {
    start: StdInstant,
    last: StdInstant,
    pub delta: StdDuration,
}

impl Default for PlayClock {
    fn default() -> Self {
        Self {
            start: StdInstant::now(),
            last: StdInstant::now(),
            delta: StdDuration::new(0, 0),
        }
    }
}
impl PlayClock {
    pub fn tick(&mut self) {
        let now = StdInstant::now();
        self.delta = now.duration_since(self.last);
        self.last = now;
    }
    pub fn last_seconds(&self) -> f32 {
        self.last.duration_since(self.start).as_secs_f32()
    }
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    pub fn last_seconds_f64(&self) -> f64 {
        self.last.duration_since(self.start).as_secs_f64()
    }
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta.as_secs_f64()
    }
}
