use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoomTemp::Table)
                    .if_not_exists()
                    .col(pk_auto(RoomTemp::Id))
                    .col(float(RoomTemp::Temp).null())
                    .col(float(RoomTemp::Humidity).null())
                    .col(float(RoomTemp::Pressure).null())
                    .col(
                        date_time(RoomTemp::UpdatedAt)
                            .not_null()
                            .extra("DEFAULT (datetime('now', 'localtime'))".to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoomTemp::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoomTemp {
    Table,
    Id,
    Temp,
    Humidity,
    Pressure,
    UpdatedAt,
}
