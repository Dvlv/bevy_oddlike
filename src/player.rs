use crate::common_components::{Velocity, AnimationHandler};
use bevy::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player)
            .add_systems(Update, (update_player, animate_player));
    }
}

const PLAYER_WALK_SPEED: f32 = 3.0;
#[derive(Debug)]
enum PlayerState {
    IDLE,
    WALKING,
}

enum PlayerMovement {
    NOTHING,
    WALK,
}

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    next_movement: PlayerMovement,
    is_facing_left: bool,
    is_accepting_input: bool,
    animation_handler: AnimationHandler,
}

/// Create the player at a default location and in an idle state
fn create_player(mut commands: Commands) {
    let player = Player {
        state: PlayerState::IDLE,
        next_movement: PlayerMovement::NOTHING,
        is_facing_left: false,
        is_accepting_input: true,
        animation_handler: AnimationHandler::new(),
    };
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let default_velo = Velocity(Vec2::new(0.0, 0.0));

    commands.spawn((player, transform, default_velo));
}

/// First we processed buffered input, then we buffer the next input
/// Or if there was no buffered input to process, we process the current input
/// and buffer any new input if we are in a busy state
fn update_player(mut query: Query<(&mut Player, &mut Transform)>, keys: Res<Input<KeyCode>>) {
    for (mut player, mut transform) in &mut query {
        if player.is_accepting_input {
            let mut was_input_pressed = false;
            // read input from keyboard and update state
            if keys.pressed(KeyCode::Right) {
                was_input_pressed = true;

                match player.state {
                    PlayerState::IDLE | PlayerState::WALKING => {
                        player.state = PlayerState::WALKING;
                        player.is_facing_left = false;
                        player.is_accepting_input = false;
                    }
                    _ => {}
                }
            } else if keys.pressed(KeyCode::Left) {
                was_input_pressed = true;

                match player.state {
                    PlayerState::IDLE | PlayerState::WALKING => {
                        player.state = PlayerState::WALKING;
                        player.is_facing_left = true;
                        player.is_accepting_input = false;
                    }
                    _ => {}
                }
            }

            if !was_input_pressed {
                player.state = PlayerState::IDLE;
            }
        } else {
            // render animation
        }
    }
}

fn animate_player(mut query: Query<(&mut AnimationHandler, &mut Player)>, time: Res<Time>) {
    for (mut anim, mut player) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current_frame += 1;
            // TODO pass num frames in
        }

    }

}