use background::BackgroundPlugin;
use bevy::prelude::*;
mod common_components;
mod constants;
mod player;
mod background;

use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}

