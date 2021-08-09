use bevy::reflect::TypeUuid;
use bevy::{ecs::component::Component, prelude::*};

use crate::MyEaser;

#[cfg(feature = "ease_handle")]
use crate::IntermediateLerp;
use crate::{
    CustomComponentEase, Ease, EaseValue, EasingChainComponent, EasingComponent, EasingState,
    EasingType,
};

#[derive(Default)]
struct HandleCache<T: 'static + TypeUuid + Sync + Send>(std::collections::HashMap<i128, Handle<T>>);

/// Plugin to add systems related to easing
#[derive(Debug, Clone, Copy)]
pub struct EasingsPlugin;

impl Plugin for EasingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ease_system::<Sprite>.system())
            .add_system(ease_system::<ColorMaterial>.system())
            .add_system(ease_system::<Color>.system())
            .add_system(ease_system::<Transform>.system())
            .add_system(ease_system::<Style>.system());

        #[cfg(feature = "ease_handle")]
        app.init_resource::<HandleCache<ColorMaterial>>()
            .add_system(handle_ease_system::<ColorMaterial>.system());
    }
}

pub fn ease_system<T: Ease + Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut T)>,
    mut easing_query: Query<&mut EasingComponent<T>>,
    mut chain_query: Query<&mut EasingChainComponent<T>>,
) where
    EaseValue<T>: interpolation::Lerp<Scalar = f32>,
    T: Default,
{
    for (entity, mut object) in query.iter_mut() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            }
            if easing.paused {
                if easing.timer.just_finished() {
                    match easing.easing_type {
                        EasingType::Once { duration } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::Loop { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::PingPong { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                    }
                    easing.timer.reset();
                    easing.paused = false;
                }
            } else {
                if easing.timer.duration().as_secs_f32() != 0. {
                    let progress = if easing.direction.is_positive() {
                        easing.timer.percent()
                    } else {
                        easing.timer.percent_left()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(start, &easing.end, &factor).0;
                    } else {
                        *object =
                            interpolation::lerp(&EaseValue(T::default()), &easing.end, &factor).0;
                    }
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<T>>();
                        }
                        EasingType::Loop { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                        }
                        EasingType::PingPong { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                            easing.direction *= -1;
                        }
                    }
                }
            }
        } else if let Ok(ref mut easing_chain) = chain_query.get_mut(entity) {
            let next = easing_chain.0.pop();
            if let Some(mut next) = next {
                if next.start.is_none() {
                    next.start = Some(EaseValue(std::mem::take(&mut object)));
                }
                if let Some(ref start) = next.start {
                    *object = interpolation::lerp(start, &next.end, &0.).0;
                } else {
                    *object = interpolation::lerp(&EaseValue(T::default()), &next.end, &0.).0;
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<T>>();
            }
        }
    }
}

/// Ease system for custom component. Add this system to your application with your component as a type parameter.
pub fn custom_ease_system<T: CustomComponentEase + Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut T)>,
    mut easing_query: Query<&mut EasingComponent<T>>,
    mut chain_query: Query<&mut EasingChainComponent<T>>,
) where
    T: interpolation::Lerp<Scalar = f32> + Default,
{
    for (entity, mut object) in query.iter_mut() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            }
            if easing.paused {
                if easing.timer.just_finished() {
                    match easing.easing_type {
                        EasingType::Once { duration } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::Loop { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::PingPong { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                    }
                    easing.timer.reset();
                    easing.paused = false;
                }
            } else {
                if easing.timer.duration().as_secs_f32() != 0. {
                    let progress = if easing.direction.is_positive() {
                        easing.timer.percent()
                    } else {
                        easing.timer.percent_left()
                    };
                    let factor = progress.compute(easing.ease_function);
                    if let Some(ref start) = easing.start {
                        *object = interpolation::lerp(&start.0, &easing.end.0, &factor);
                    } else {
                        *object = interpolation::lerp(&T::default(), &easing.end.0, &factor);
                    }
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<T>>();
                        }
                        EasingType::Loop { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                        }
                        EasingType::PingPong { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                            easing.direction *= -1;
                        }
                    }
                }
            }
        } else if let Ok(ref mut easing_chain) = chain_query.get_mut(entity) {
            let next = easing_chain.0.pop();
            if let Some(mut next) = next {
                if next.start.is_none() {
                    next.start = Some(EaseValue(std::mem::take(&mut object)));
                }
                if let Some(ref start) = next.start {
                    *object = interpolation::lerp(&start.0, &next.end.0, &0.);
                } else {
                    *object = interpolation::lerp(&T::default(), &next.end.0, &0.);
                }

                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<T>>();
            }
        }
    }
}

#[cfg(feature = "ease_handle")]
#[allow(clippy::too_many_arguments)]
fn handle_ease_system<T: Ease + Component + TypeUuid>(
    mut commands: Commands,
    time: Res<Time>,
    mut assets: ResMut<Assets<T>>,
    mut handle_cache: ResMut<HandleCache<T>>,
    mut query: Query<(Entity, &mut Handle<T>)>,
    mut easing_query: Query<&mut EasingComponent<Handle<T>>>,
    mut chain_query: Query<&mut EasingChainComponent<Handle<T>>>,
) where
    T: IntermediateLerp,
{
    for (entity, mut object) in query.iter_mut() {
        if let Ok(ref mut easing) = easing_query.get_mut(entity) {
            if easing.state == EasingState::Play {
                easing.timer.tick(time.delta());
            }
            if easing.paused {
                if easing.timer.just_finished() {
                    match easing.easing_type {
                        EasingType::Once { duration } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::Loop { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                        EasingType::PingPong { duration, .. } => {
                            easing.timer.set_duration(duration);
                        }
                    }
                    easing.timer.reset();
                    easing.paused = false;
                }
            } else {
                if easing.timer.duration().as_secs_f32() != 0. {
                    let progress = if easing.direction.is_positive() {
                        easing.timer.percent()
                    } else {
                        easing.timer.percent_left()
                    };
                    let factor = progress.compute(easing.ease_function);
                    let factor_simplified = (factor * 25.) as i16;
                    let handle = handle_cache
                        .0
                        .entry(easing.id + (easing.direction * factor_simplified) as i128)
                        .or_insert_with(|| {
                            let start = assets.get(&easing.start.as_ref().unwrap().0).unwrap();
                            let end = assets.get(&easing.end.0).unwrap();
                            let intermediate =
                                IntermediateLerp::lerp(&EaseValue(start), &EaseValue(end), &factor);

                            assets.add(intermediate)
                        })
                        .clone();
                    *object = handle;
                }
                if easing.timer.finished() {
                    match easing.easing_type {
                        EasingType::Once { .. } => {
                            commands.entity(entity).remove::<EasingComponent<T>>();
                        }
                        EasingType::Loop { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                        }
                        EasingType::PingPong { pause, .. } => {
                            if let Some(pause) = pause {
                                easing.timer.set_duration(pause);
                                easing.paused = true;
                            }
                            easing.timer.reset();
                            easing.direction *= -1;
                        }
                    }
                }
            }
        } else if let Ok(ref mut easing_chain) = chain_query.get_mut(entity) {
            let next = easing_chain.0.pop();
            if let Some(mut next) = next {
                if next.start.is_none() {
                    next.start = Some(EaseValue(object.clone()));
                }
                commands.entity(entity).insert(next);
            } else {
                commands.entity(entity).remove::<EasingChainComponent<T>>();
            }
        }
    }
}
