use bevy::prelude::*;
use dashboard::bme::BmePlugin;
use dashboard::db::{dbconn, DBContainer};


#[tokio::main]
async fn main() {
    let db = dbconn().await.expect("DB接続失敗");

    App::new()
        .insert_resource(DBContainer { db })
        .add_plugins(DefaultPlugins)
        .add_plugins(BmePlugin)
        .run();
}
