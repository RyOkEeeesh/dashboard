use bevy::prelude::*;
use dashboard::bme::BmePlugin;
use dashboard::db::{DbManager, DbSender};

#[tokio::main]
async fn main() {
    let (db_manager, rx) = DbManager::new();

    DbManager::run_task(rx);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DbSender(db_manager.tx))
        .add_plugins(BmePlugin)
        .run();
}
