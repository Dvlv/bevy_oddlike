use bevy::prelude::*;
mod common_components;
mod player;

use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .run();
}
