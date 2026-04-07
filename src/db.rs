use bevy::prelude::*;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

#[derive(Resource)]
pub struct DBContainer {
    pub db: DatabaseConnection,
}


pub async fn dbconn() -> Result<DatabaseConnection, DbErr>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:local.db?mode=rwc".to_string());

    let db = Database::connect(&db_url).await?;
    println!("Connected DB");
    Ok(db)
}