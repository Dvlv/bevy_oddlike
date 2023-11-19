use bevy::prelude::*;
mod common_components;
mod player;
mod constants;

use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .run();
}
