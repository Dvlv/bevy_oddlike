use crate::common_components::{AnimationHandler, Velocity};
use crate::constants::NUM_TILES;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player)
            .add_systems(Update, (update_player, animate_player));
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum PlayerState {
    IDLE,
    WALKING,
    HOPPING,
    CROUCHING,
    CROUCHED,
    UNCROUCHING,
    ROLLING,
    RUNNING,
    RUNJUMPING,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum PlayerMovement {
    NOTHING,
    HOP,
    CROUCH,
    UNCROUCH,
    RUNJUMP,
}

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    next_movement: PlayerMovement,
    is_facing_left: bool,
    animation_handler: AnimationHandler,
    state_frames: HashMap<PlayerState, (usize, usize)>,
}

/// Create the player at a default location and in an idle state
fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut player_frames = HashMap::<PlayerState, (usize, usize)>::new();
    player_frames.insert(PlayerState::IDLE, (0 as usize, 0 as usize));
    player_frames.insert(PlayerState::WALKING, (0 as usize, 3 as usize));
    player_frames.insert(PlayerState::HOPPING, (4 as usize, 11 as usize));
    player_frames.insert(PlayerState::CROUCHING, (12 as usize, 14 as usize));
    player_frames.insert(PlayerState::CROUCHED, (15 as usize, 15 as usize));
    player_frames.insert(PlayerState::UNCROUCHING, (15 as usize, 18 as usize));
    player_frames.insert(PlayerState::ROLLING, (19 as usize, 21 as usize));
    player_frames.insert(PlayerState::RUNNING, (22 as usize, 25 as usize));
    player_frames.insert(PlayerState::RUNJUMPING, (6 as usize, 9 as usize));

    let player = Player {
        state: PlayerState::IDLE,
        next_movement: PlayerMovement::NOTHING,
        is_facing_left: false,
        animation_handler: AnimationHandler::new(0, 0), // IDLE is 1 frame
        state_frames: player_frames,
    };
    let default_velo = Velocity(Vec2::new(0.0, 0.0));

    let texture_handle = asset_server.load("player-walk.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 12, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_sprite_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(1),
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((player, player_sprite_sheet, default_velo));
}

/// First we processed buffered input, then we buffer the next input
/// Or if there was no buffered input to process, we process the current input
/// and buffer any new input if we are in a busy state
fn update_player(
    mut query: Query<(&mut Player, &mut TextureAtlasSprite)>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut player, mut texture) in &mut query {
        let mut was_input_pressed = false;
        if !player.animation_handler.is_playing {
            if player.next_movement == PlayerMovement::HOP {
                // buffered hop, ignore other input for now
                change_player_state(&mut player, PlayerState::HOPPING);
                player.next_movement = PlayerMovement::NOTHING;
            } else if player.next_movement == PlayerMovement::RUNJUMP {
                change_player_state(&mut player, PlayerState::RUNJUMPING);
                player.next_movement = PlayerMovement::NOTHING;
            } else if player.next_movement == PlayerMovement::CROUCH
                && player.state != PlayerState::CROUCHING
            {
                change_player_state(&mut player, PlayerState::CROUCHING);
                player.next_movement = PlayerMovement::NOTHING;
            } else if player.next_movement == PlayerMovement::UNCROUCH {
                change_player_state(&mut player, PlayerState::UNCROUCHING);
                player.next_movement = PlayerMovement::NOTHING;
            } else {
                // read input from keyboard and update state
                if keys.pressed(KeyCode::Right) {
                    was_input_pressed = true;

                    match player.state {
                        PlayerState::IDLE
                        | PlayerState::WALKING
                        | PlayerState::HOPPING
                        | PlayerState::RUNNING => {
                            if keys.pressed(KeyCode::S) {
                                change_player_state(&mut player, PlayerState::RUNNING);
                                player.is_facing_left = false;
                            } else {
                                change_player_state(&mut player, PlayerState::WALKING);
                                player.is_facing_left = false;
                            }
                        }
                        PlayerState::CROUCHED | PlayerState::ROLLING => {
                            change_player_state(&mut player, PlayerState::ROLLING);
                            player.is_facing_left = false;
                        }
                        _ => {
                            was_input_pressed = false;
                        }
                    }
                } else if keys.pressed(KeyCode::Left) {
                    was_input_pressed = true;

                    match player.state {
                        PlayerState::IDLE
                        | PlayerState::WALKING
                        | PlayerState::HOPPING
                        | PlayerState::RUNNING => {
                            if keys.pressed(KeyCode::S) {
                                change_player_state(&mut player, PlayerState::RUNNING);
                                player.is_facing_left = true;
                            } else {
                                change_player_state(&mut player, PlayerState::WALKING);
                                player.is_facing_left = true;
                            }
                        }
                        PlayerState::CROUCHED | PlayerState::ROLLING => {
                            change_player_state(&mut player, PlayerState::ROLLING);
                            player.is_facing_left = true;
                        }
                        _ => {
                            was_input_pressed = false;
                        }
                    }
                } else if keys.pressed(KeyCode::Down) {
                    was_input_pressed = true;
                    match player.state {
                        PlayerState::IDLE | PlayerState::CROUCHING => {
                            change_player_state(&mut player, PlayerState::CROUCHING);
                        }
                        _ => {
                            was_input_pressed = false;
                        }
                    }
                } else if keys.pressed(KeyCode::Up) {
                    was_input_pressed = true;
                    match player.state {
                        PlayerState::CROUCHED => {
                            change_player_state(&mut player, PlayerState::UNCROUCHING);
                        }
                        _ => {
                            was_input_pressed = false;
                        }
                    }
                }
                if keys.just_pressed(KeyCode::A) {
                    was_input_pressed = true;
                    change_player_state(&mut player, PlayerState::HOPPING);
                }

                texture.flip_x = player.is_facing_left;

                let is_crouching = player.state == PlayerState::CROUCHING
                    || player.state == PlayerState::CROUCHED
                    || player.state == PlayerState::ROLLING;

                if !was_input_pressed && !is_crouching {
                    change_player_state(&mut player, PlayerState::IDLE);
                } else if !was_input_pressed && is_crouching {
                    change_player_state(&mut player, PlayerState::CROUCHED);
                }
            }
        } else {
            // rendering animation
            if keys.just_pressed(KeyCode::A) {
                //change_player_state(&mut player, PlayerState::HOPPING);
                if player.state != PlayerState::RUNNING {
                    player.next_movement = PlayerMovement::HOP;
                } else {
                    player.next_movement = PlayerMovement::RUNJUMP;
                }
            } else if keys.just_pressed(KeyCode::Down) {
                player.next_movement = PlayerMovement::CROUCH;
            } else if keys.just_pressed(KeyCode::Up) {
                if player.state == PlayerState::CROUCHED || player.state == PlayerState::ROLLING {
                    player.next_movement = PlayerMovement::UNCROUCH;
                }
            }
        }
    }
}

fn change_player_state(player: &mut Player, state: PlayerState) {
    player.state = state;
    player.animation_handler.is_playing = true;
    player.animation_handler.min_frame = player.state_frames[&player.state].0;
    player.animation_handler.max_frame = player.state_frames[&player.state].1;
    player.animation_handler.current_frame = player.animation_handler.min_frame;

    if state == PlayerState::RUNNING {
        player.animation_handler.set_timer_speed(0.05);
    } else {
        player.animation_handler.set_timer_speed(0.1);
    }
}

fn animate_player(
    mut query: Query<(&mut TextureAtlasSprite, &mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut spritesheet, mut player, mut transform) in &mut query {
        let player_current_state = &player.state.clone();
        let is_left = &player.is_facing_left.clone();
        let anim_bounds = &player.state_frames[player_current_state].clone();
        let anim = &mut player.animation_handler;

        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current_frame += 1;

            // if walk / run / hop, move forwards
            if player_current_state == &PlayerState::WALKING
                || player_current_state == &PlayerState::RUNNING
                || player_current_state == &PlayerState::RUNJUMPING
                || player_current_state == &PlayerState::HOPPING
                || player_current_state == &PlayerState::ROLLING
            {
                // let mut move_step = window / NUM_TILES;
                let window = window_query.get_single().unwrap();
                let num_frames: f32 = anim.max_frame as f32 - anim.min_frame as f32 + 1 as f32;
                let move_step = window.width() / NUM_TILES / num_frames;

                if player_current_state == &PlayerState::WALKING
                    || player_current_state == &PlayerState::RUNNING
                    || player_current_state == &PlayerState::ROLLING
                {
                    // if walking, move fwd on all frames
                    if *is_left {
                        transform.translation.x -= move_step;
                    } else {
                        transform.translation.x += move_step;
                    }
                } else if player_current_state == &PlayerState::HOPPING {
                    // if hopping, we only move on the middle frames
                    // TODO unhardcode these somehow
                    if anim.current_frame == 7 || anim.current_frame == 8 {
                        transform.translation.y += 20.0;
                    } else if anim.current_frame == 9 || anim.current_frame == 10 {
                        transform.translation.y -= 20.0;
                    }

                    if anim.current_frame > 6 && anim.current_frame < 11 {
                        let move_step = move_step * 3.0;
                        // TODO can I refactor this out?
                        if *is_left {
                            transform.translation.x -= move_step;
                        } else {
                            transform.translation.x += move_step;
                        }
                    }
                } else if player_current_state == &PlayerState::RUNJUMPING {
                    // if hopping, we only move on the middle frames
                    // TODO unhardcode these somehow
                    if anim.current_frame == 7 {
                        transform.translation.y += 30.0;
                    } else if anim.current_frame == 9 {
                        transform.translation.y -= 30.0;
                    }

                    let move_step = move_step * 3.0;
                    // TODO can I refactor this out?
                    if *is_left {
                        transform.translation.x -= move_step;
                    } else {
                        transform.translation.x += move_step;
                    }
                }
            }

            if anim.current_frame > anim_bounds.1 {
                match player_current_state {
                    &PlayerState::WALKING | &PlayerState::ROLLING | &PlayerState::RUNNING => {
                        anim.current_frame = anim_bounds.0;
                    }
                    _ => {
                        anim.current_frame = anim_bounds.1;
                    }
                }
                anim.is_playing = false;
            }
        }

        spritesheet.index = anim.current_frame;
        if player_current_state == &PlayerState::IDLE {
            anim.is_playing = false;
        }
    }
}
