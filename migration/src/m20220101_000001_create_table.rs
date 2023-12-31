use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Services::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Services::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Services::Detail).string().not_null())
                    .col(ColumnDef::new(Services::Price).string().not_null())
                    .col(ColumnDef::new(Services::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Services::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Services::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Services {
    Table,
    Id,
    Detail,
    Price,
    Uuid,
    CreatedAt,
}
