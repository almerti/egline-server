use sea_orm_migration::prelude::*;

use super::m20240427_083430_create_table_book::Book;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chapter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chapter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chapter::BookId).integer().not_null())
                    .col(ColumnDef::new(Chapter::Title).string().not_null())
                    .col(ColumnDef::new(Chapter::Filepath).string().not_null())
                    .col(ColumnDef::new(Chapter::Number).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chapter-book_id")
                            .from(Chapter::Table, Chapter::BookId)
                            .to(Book::Table, Book::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chapter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chapter {
    Table,
    Id,
    BookId,
    Title,
    Filepath,
    Number,
}
