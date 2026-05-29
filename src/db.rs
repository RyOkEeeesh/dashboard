use chrono::{FixedOffset, TimeZone};
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, EntityTrait, NotSet, Set,
    Statement,
};
use std::{fs, path::Path};
use tokio::sync::{mpsc, oneshot};

use crate::bme::WeatherData;
use crate::entities::{app_slot, room_temp};
use crate::ui::AppData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date(pub i32, pub u32, pub u32);

pub enum DbRequest {
    SetTemp(WeatherData),
    GetTemp(oneshot::Sender<Vec<room_temp::Model>>, Date),
    SetApps(Vec<AppData>),
    GetApps(oneshot::Sender<Vec<app_slot::Model>>),
}

pub fn db_run(mut rx: mpsc::Receiver<DbRequest>) {
    tokio::spawn(async move {
        let db = dbconn().await.expect("Failed to connect DB");
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
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
                DbRequest::GetTemp(respond_to, Date(year, month, day)) => {
                    let start_date = jst.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
                    let end_date = start_date + chrono::Duration::days(1);

                    let start_str = start_date.format("%Y-%m-%d %H:%M:%S").to_string();
                    let end_str = end_date.format("%Y-%m-%d %H:%M:%S").to_string();

                    let history = room_temp::Entity::find()
                        .from_raw_sql(Statement::from_sql_and_values(
                            DbBackend::Sqlite,
                            r#"
                                SELECT * FROM room_temp 
                                WHERE updated_at IN (
                                    SELECT MIN(updated_at)
                                    FROM room_temp
                                    WHERE updated_at >= $1 AND updated_at < $2
                                    GROUP BY strftime('%Y-%m-%d %H', updated_at)
                                )
                                ORDER BY updated_at ASC
                                "#,
                            [start_str.into(), end_str.into()],
                        ))
                        .all(&db)
                        .await;

                    if let Ok(data) = history {
                        let _ = respond_to.send(data);
                    }
                }
                DbRequest::SetApps(app_data) => {
                    if app_data.is_empty() {
                        return;
                    }

                    let mut data: Vec<app_slot::ActiveModel> = Vec::new();
                    for app in app_data.iter() {
                        let app_name_str = format!("{:?}", app.app_type);
                        data.push(app_slot::ActiveModel {
                            id: NotSet,
                            app_name: Set(app_name_str),
                            slot: Set(app.slot),
                        });
                    }

                    let on_conflict = OnConflict::column(app_slot::Column::AppName)
                        .update_column(app_slot::Column::Slot)
                        .to_owned();

                    let _ = app_slot::Entity::insert_many(data)
                        .on_conflict(on_conflict)
                        .exec(&db)
                        .await;
                }
                DbRequest::GetApps(respond_to) => {
                    let app_slot = app_slot::Entity::find().all(&db).await;
                    if let Ok(data) = app_slot {
                        let _ = respond_to.send(data);
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
            db.execute_unprepared(sql).await?;
            Ok(db)
        }
    }
}
