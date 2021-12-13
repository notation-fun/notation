#[cfg(target_arch = "wasm32")]
use instant::Duration as StdDuration;
#[cfg(target_arch = "wasm32")]
use instant::Instant as StdInstant;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration as StdDuration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;

pub struct PlayClock {
    last: StdInstant,
    pub delta: StdDuration,
}

impl Default for PlayClock {
    fn default() -> Self {
        Self {
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
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    pub fn get_now() {
        StdInstant::now();
    }
}
