use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait, NotSet, Set};
use tokio::sync::{mpsc, oneshot};

use crate::bme::WeatherData;
use crate::entities::room_temp;

#[derive(Resource)]
pub struct DbSender(pub mpsc::Sender<DbRequest>);

#[derive(Resource)]
struct DbReceiver(Option<mpsc::Receiver<DbRequest>>);

pub struct DbPlugin;

impl Plugin for DbPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel::<DbRequest>(100);

        app.insert_resource(DbSender(tx))
            .insert_resource(DbReceiver(Some(rx))) // Resourceとして登録
            .add_systems(Startup, setup); // 普通のシステムとして登録
    }
}
pub enum DbRequest {
    SaveWeather(WeatherData),
    GetHistory(oneshot::Sender<Vec<room_temp::Model>>),
}

fn setup(runtime: ResMut<TokioTasksRuntime>, mut receiver: ResMut<DbReceiver>) {
    // ResourceからReceiverを取り出す（一度きり）
    if let Some(rx) = receiver.0.take() {
        runtime.spawn_background_task(|_ctx| async move {
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
                    DbRequest::GetHistory(respond_to) => {
                        if let Ok(history) = room_temp::Entity::find().all(&db).await {
                            let _ = respond_to.send(history);
                        }
                    }
                }
            }
        });
    }
}

async fn dbconn() -> Result<DatabaseConnection, DbErr> {
    let db_path = "local.db";
    let db_url = format!("sqlite:{db_path}");

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
