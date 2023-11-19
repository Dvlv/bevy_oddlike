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

    let player = Player {
        state: PlayerState::IDLE,
        next_movement: PlayerMovement::NOTHING,
        is_facing_left: false,
        animation_handler: AnimationHandler::new(0, 0), // IDLE is 1 frame
        state_frames: player_frames,
    };
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let default_velo = Velocity(Vec2::new(0.0, 0.0));

    let texture_handle = asset_server.load("player-walk.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 5, 1, None, None);
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
    mut query: Query<(&mut Player, &mut Transform, &mut TextureAtlasSprite)>,
    keys: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut player, mut transform, mut texture) in &mut query {
        if !player.animation_handler.is_playing {
            let mut was_input_pressed = false;
            // read input from keyboard and update state
            if keys.pressed(KeyCode::Right) {
                was_input_pressed = true;

                match player.state {
                    PlayerState::IDLE | PlayerState::WALKING => {
                        change_player_state(&mut player, PlayerState::WALKING);
                        player.is_facing_left = false;
                    }
                    _ => {}
                }
            } else if keys.pressed(KeyCode::Left) {
                was_input_pressed = true;

                match player.state {
                    PlayerState::IDLE | PlayerState::WALKING => {
                        change_player_state(&mut player, PlayerState::WALKING);
                        player.is_facing_left = true;
                    }
                    _ => {}
                }
            }

            texture.flip_x = player.is_facing_left;

            if !was_input_pressed {
                player.state = PlayerState::IDLE;
            }
        } else {
            // render animation
        }
    }
}

fn change_player_state(mut player: &mut Player, state: PlayerState) {
    player.state = state;
    player.animation_handler.is_playing = true;
    player.animation_handler.min_frame = player.state_frames[&player.state].0;
    player.animation_handler.max_frame = player.state_frames[&player.state].1;
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

            if player_current_state == &PlayerState::WALKING {
                // let mut move_step = window / NUM_TILES;
                let window = window_query.get_single().unwrap();
                let num_frames: f32 = anim.max_frame as f32 - anim.min_frame as f32 + 1 as f32;
                let move_step = window.width() / NUM_TILES / num_frames;

                if *is_left {
                    transform.translation.x -= move_step;
                } else {
                    transform.translation.x += move_step;
                }
            }

            if anim.current_frame > anim_bounds.1 {
                anim.current_frame = anim_bounds.0;
                anim.is_playing = false;
            }
        }

        spritesheet.index = anim.current_frame;
    }
}
