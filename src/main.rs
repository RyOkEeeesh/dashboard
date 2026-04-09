use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksPlugin;
use dashboard::bme::BmePlugin;
use dashboard::clock::ClockSysPlugin;
use dashboard::db::{DbPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ClockSysPlugin))
        .add_plugins(TokioTasksPlugin::default())
        .add_plugins((DbPlugin, BmePlugin))
        .run();
}
