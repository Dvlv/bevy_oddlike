use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);


#[derive(Component)]
pub struct AnimationHandler {
    pub current_frame: usize,
    pub timer: Timer,
}

impl AnimationHandler {
    pub fn new() -> Self {
        AnimationHandler {
            current_frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}