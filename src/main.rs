use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksPlugin;
use dashboard::bme::BmePlugin;
use dashboard::clock::ClockSysPlugin;
use dashboard::db::{DbRequest, DbSender, db_run};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<DbRequest>(100);

    db_run(rx);

    App::new()
        .add_plugins((DefaultPlugins, ClockSysPlugin))
        // .add_plugins(TokioTasksPlugin::default())
        .insert_resource(DbSender(tx))
        .add_plugins(BmePlugin)
        .run();
}
