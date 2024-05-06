use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Book::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Description).string().not_null())
                    .col(ColumnDef::new(Book::Cover).binary().not_null())
                    .col(ColumnDef::new(Book::Rating).float().not_null())
                    .col(ColumnDef::new(Book::Date).date().not_null())
                    .col(ColumnDef::new(Book::Views).integer().not_null())
                    .col(ColumnDef::new(Book::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Book {
    Table,
    Id,
    Title,
    Description,
    Cover,
    Rating,
    Date,
    Views,
    Status
}
