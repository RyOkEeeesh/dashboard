use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait, NotSet, Set, Statement,
};
use std::{fs, path::Path};
use tokio::sync::{mpsc, oneshot};

use crate::bme::WeatherData;
use crate::entities::room_temp;

pub struct Date {
    pub y: i32,
    pub m: i32,
    pub d: i32,
}

pub enum DbRequest {
    SetTemp(WeatherData),
    GetTemp(oneshot::Sender<Vec<room_temp::Model>>, Date),
}

pub fn db_run(mut rx: mpsc::Receiver<DbRequest>) {
    tokio::spawn(async move {
        let db = dbconn().await.expect("Failed to connect DB");
        while let Some(request) = rx.recv().await {
            match request {
                DbRequest::SetTemp(data) => {
                    if matches!(
                        data,
                        WeatherData {
                            temp: None,
                            humidity: None,
                            pressure: None
                        }
                    ) {
                        continue;
                    }
                    let active_model = room_temp::ActiveModel {
                        id: NotSet,
                        temp: Set(data.temp),
                        humidity: Set(data.humidity),
                        pressure: Set(data.pressure),
                        updated_at: NotSet,
                    };
                    let _ = room_temp::Entity::insert(active_model).exec(&db).await;
                }
                DbRequest::GetTemp(respond_to, date) => {
                    if let Ok(history) = room_temp::Entity::find().all(&db).await {
                        let _ = respond_to.send(history);
                    }
                }
            }
        }
    });
}

async fn dbconn() -> Result<DatabaseConnection, DbErr> {
    let db_path = if cfg!(debug_assertions) {
        "db/local.db"
    } else {
        "/var/lib/dashboard/local.db"
    };
    let db_url = format!("sqlite:{db_path}");

    match Database::connect(&db_url).await {
        Ok(db) => Ok(db),
        Err(_) => {
            if let Some(parent) = Path::new(db_path).parent() {
                let _ = fs::create_dir_all(parent);
            }

            let _ = fs::File::create(db_path);

            let db = Database::connect(&db_url).await?;

            let sql = include_str!("../db/setting.sql");
            db.execute(Statement::from_string(db.get_database_backend(), sql))
                .await?;

            Ok(db)
        }
    }
}
