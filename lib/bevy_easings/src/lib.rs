#![deny(
    warnings,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    //unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
//#![feature(float_interpolation)]

//! Ease plugin for Bevy

use std::time::Duration;

#[cfg(feature = "ease_handle")]
use rand::Rng;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use interpolation::Ease as IEase;
pub use interpolation::{EaseFunction, Lerp};

mod plugin;
pub use plugin::{custom_ease_system, EasingsPlugin};
mod implemented;

/// Wrapper around a type that can be eased.
#[derive(Debug)]
pub struct EaseValue<T>(pub T);

/// How should this easing loop repeat
#[derive(Clone, Copy)]
pub enum EasingType {
    /// Only happen once
    Once {
        /// duration of the easing
        duration: Duration,
    },
    /// Looping, restarting from the start once finished
    Loop {
        /// duration of the easing
        duration: Duration,
        /// duration of the pause between two loops
        pause: Option<Duration>,
    },
    /// Repeat the animation back and forth
    PingPong {
        /// duration of the easing
        duration: Duration,
        /// duration of the pause before starting again in the other direction
        pause: Option<Duration>,
    },
}

/// Control if an easing is played
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EasingState {
    /// Play the easing
    Play,
    /// Pause the easing
    Paused,
}

impl std::ops::Not for EasingState {
    type Output = EasingState;

    fn not(self) -> Self::Output {
        match self {
            EasingState::Paused => EasingState::Play,
            EasingState::Play => EasingState::Paused,
        }
    }
}

/// Describe how eased value should be computed
#[derive(Clone, Copy)]
pub enum EaseMethod {
    /// Follow `EaseFunction`
    EaseFunction(EaseFunction),
    /// Linear interpolation, with no function
    Linear,
    /// Discrete interpolation, eased value will jump from start to end
    Discrete,
    /// Use a custom function to interpolate the value
    CustomFunction(fn(f32) -> f32),
}

impl Into<EaseMethod> for EaseFunction {
    fn into(self) -> EaseMethod {
        EaseMethod::EaseFunction(self)
    }
}

trait MyEaser {
    fn compute(self, function: EaseMethod) -> Self;
}
impl MyEaser for f32 {
    fn compute(self, function: EaseMethod) -> f32 {
        match function {
            EaseMethod::EaseFunction(function) => self.calc(function),
            EaseMethod::Linear => {
                let delta = 0.01;
                if self < 0. + delta {
                    0.
                } else if self > 1. - delta {
                    1.
                } else {
                    self
                }
            }
            EaseMethod::Discrete => {
                if self > 0.5 {
                    1.
                } else {
                    0.
                }
            }
            EaseMethod::CustomFunction(function) => function(self),
        }
    }
}

/// Component to control an easing
pub struct EasingComponent<T> {
    start: Option<EaseValue<T>>,
    end: EaseValue<T>,
    ease_function: EaseMethod,
    timer: Timer,
    /// Control if this easing is played or not
    pub state: EasingState,
    paused: bool,
    easing_type: EasingType,
    #[cfg(feature = "ease_handle")]
    id: i128,
    direction: i16,
}

impl<T: std::fmt::Debug> std::fmt::Debug for EasingComponent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EasingComponent")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("state", &self.state)
            .finish()
    }
}

impl<T: Default> EasingComponent<T> {
    /// Start a chain of easing, adding a new one after the first one
    pub fn ease_to(
        self,
        end: T,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingChainComponent<T> {
        #[cfg(feature = "ease_handle")]
        let mut rng = rand::thread_rng();

        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration } => Timer::new(duration, false),
                EasingType::Loop { duration, .. } => Timer::new(duration, false),
                EasingType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            #[cfg(feature = "ease_handle")]
            id: rng.gen(),
            direction: 1,
        };

        EasingChainComponent(vec![next, self])
    }
}

/// Component to control a chain of easing
pub struct EasingChainComponent<T>(Vec<EasingComponent<T>>);

impl<T: Default> EasingChainComponent<T> {
    /// Add a new easing at the end of the current chain
    pub fn ease_to(
        mut self,
        end: T,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingChainComponent<T> {
        #[cfg(feature = "ease_handle")]
        let mut rng = rand::thread_rng();

        let next = EasingComponent {
            start: None,
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration } => Timer::new(duration, false),
                EasingType::Loop { duration, .. } => Timer::new(duration, false),
                EasingType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            #[cfg(feature = "ease_handle")]
            id: rng.gen(),
            direction: 1,
        };

        self.0.insert(0, next);
        self
    }
}

/// Trait marking components that can be eased
pub trait Ease: Sized {
    /// Create a new easing. If no start is provided, it will try to use the current value of the component for the target entity
    fn ease(
        start: Option<Self>,
        end: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        #[cfg(feature = "ease_handle")]
        let mut rng = rand::thread_rng();

        EasingComponent {
            start: start.map(EaseValue),
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration } => Timer::new(duration, false),
                EasingType::Loop { duration, .. } => Timer::new(duration, false),
                EasingType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            #[cfg(feature = "ease_handle")]
            id: rng.gen(),
            direction: 1,
        }
    }

    /// Create a new easing with the current component value as a starting point
    fn ease_to(
        self,
        target: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        Self::ease(Some(self), target, ease_function, easing_type)
    }
}

impl<T> Ease for EaseValue<T> where T: Lerp<Scalar = f32> {}
impl<T: 'static + TypeUuid + Send + Sync> Ease for Handle<T> where EaseValue<T>: Lerp<Scalar = f32> {}
impl<T> Ease for T where EaseValue<T>: Lerp<Scalar = f32> {}

impl<T> Default for EaseValue<T>
where
    T: Default,
{
    fn default() -> Self {
        EaseValue(T::default())
    }
}

trait IntermediateLerp: Sized {
    fn lerp(start: &EaseValue<&Self>, end: &EaseValue<&Self>, scalar: &f32) -> Self;
}

/// Trait to mark custom component that can be eased. It will be automatically implemented if the custom component implement `Lerp`
pub trait CustomComponentEase: Sized {
    /// Create a new easing. If no start is provided, it will try to use the current value of the component for the target entity
    fn ease(
        start: Option<Self>,
        end: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        #[cfg(feature = "ease_handle")]
        let mut rng = rand::thread_rng();

        EasingComponent {
            start: start.map(EaseValue),
            end: EaseValue(end),
            ease_function: ease_function.into(),
            timer: match easing_type {
                EasingType::Once { duration } => Timer::new(duration, false),
                EasingType::Loop { duration, .. } => Timer::new(duration, false),
                EasingType::PingPong { duration, .. } => Timer::new(duration, false),
            },
            state: EasingState::Play,
            paused: false,
            easing_type,
            #[cfg(feature = "ease_handle")]
            id: rng.gen(),
            direction: 1,
        }
    }

    /// Create a new easing with the current component value as a starting point
    fn ease_to(
        self,
        target: Self,
        ease_function: impl Into<EaseMethod>,
        easing_type: EasingType,
    ) -> EasingComponent<Self> {
        Self::ease(Some(self), target, ease_function, easing_type)
    }
}

impl<T> CustomComponentEase for T where T: Lerp<Scalar = f32> {}
