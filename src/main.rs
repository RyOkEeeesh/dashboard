// use std::thread;
// use std::time::Duration;

use bevy::prelude::*;

use dashboard::bme::{BmePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BmePlugin)
        .run();
}