use bevy::prelude::{Resource};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr, NotSet, Set, EntityTrait};
use std::env;
use tokio::sync::{mpsc, oneshot};

use crate::bme::WeatherData;
use crate::entities::{room_temp};

#[derive(Resource)]
pub struct DbSender(pub mpsc::Sender<DbRequest>);

pub enum DbRequest {
    SaveWeather(WeatherData),
}
pub struct DbManager {
    // 送信機だけを保持する（これはいくらでもコピーして配れる）
    pub tx: mpsc::Sender<DbRequest>,
}

impl DbManager {
    // 1. まずはチャンネルを作って、送信機を持つマネージャーを返す
    pub fn new() -> (Self, mpsc::Receiver<DbRequest>) {
        let (tx, rx) = mpsc::channel::<DbRequest>(100);
        (Self { tx }, rx)
    }

    // 2. 接続とループの開始（これは main や Plugin で一回だけ呼ぶ）
    pub fn run_task(rx: mpsc::Receiver<DbRequest>) {
        tokio::spawn(async move {
            // ここで接続（staticメソッドとして呼ぶ）
            let db = Self::dbconn().await.expect("Failed to connect DB");

            let mut receiver = rx;
            while let Some(request) = receiver.recv().await {
                match request {
                    DbRequest::SaveWeather(data) => {
                        let result = room_temp::ActiveModel {
                            id: NotSet, // IDはDBが自動採番するので NotSet
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
        let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:local.db?mode=rwc".to_string());
        Database::connect(&db_url).await
    }
}
