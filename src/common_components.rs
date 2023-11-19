use bevy::prelude::*;
use bevy::utils::HashMap;
use std::cmp::max;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct AnimationHandler {
    pub current_frame: usize,
    pub timer: Timer,
    pub is_playing: bool,
    pub min_frame: usize,
    pub max_frame: usize,
}

impl AnimationHandler {
    pub fn new(min_frame: usize, max_frame: usize) -> Self {
        AnimationHandler {
            current_frame: 0,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            is_playing: false,
            min_frame,
            max_frame,
        }
    }
}