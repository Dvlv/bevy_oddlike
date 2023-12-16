use crate::common_components::{AnimationHandler, Velocity};
use crate::constants::NUM_TILES;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_background);
            //.add_systems(Update, (update_player, animate_player));
    }
}

fn draw_background(

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bg_img = asset_server.load("bg.png");
    commands.spawn(SpriteBundle {
        texture: bg_img,
        ..default()
    });
}