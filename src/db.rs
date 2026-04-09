use bevy::prelude::Resource;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait, NotSet, Set};
use tokio::sync::{mpsc, oneshot};

use crate::bme::WeatherData;
use crate::entities::room_temp;

#[derive(Resource)]
pub struct DbSender(pub mpsc::Sender<DbRequest>);

pub enum DbRequest {
    SaveWeather(WeatherData),
}

pub fn db_run(rx: mpsc::Receiver<DbRequest>) {
    tokio::spawn(async move {
        let db = dbconn().await.expect("Failed to connect DB");

        let mut receiver = rx;
        while let Some(request) = receiver.recv().await {
            match request {
                DbRequest::SaveWeather(data) => {
                    let result = room_temp::ActiveModel {
                        id: NotSet,
                        temp: Set(data.temperature),
                        humidity: Set(data.humidity),
                        pressure: Set(data.pressure),
                        updated_at: NotSet,
                    };
                    let _ = room_temp::Entity::insert(result).exec(&db).await;
                }
            }
        }
    });
}

async fn dbconn() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let db_path = "local.db";
    let db_url = format!("sqlite:{}", db_path);

    match Database::connect(&db_url).await {
        Ok(db) => Ok(db),
        Err(_) => {
            let _ = std::fs::File::create(db_path);

            let db = Database::connect(&db_url).await?;

            let sql = r#"
                CREATE TABLE room_temp (
                    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                    temp FLOAT,
                    humidity FLOAT,
                    pressure FLOAT,
                    updated_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
                );
                "#;

            use sea_orm::{ConnectionTrait, Statement};
            db.execute(Statement::from_string(db.get_database_backend(), sql))
                .await?;

            Ok(db)
        }
    }
}
