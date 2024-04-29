use sea_orm_migration::prelude::*;

use super::m20240427_083430_create_table_book::Book;
use super::m20240427_164712_create_table_author::Author;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookAuthor::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(BookAuthor::Table)
                            .col(BookAuthor::BookId)
                            .col(BookAuthor::AuthorId)
                    )
                    .col(ColumnDef::new(BookAuthor::BookId).integer().not_null())
                    .col(ColumnDef::new(BookAuthor::AuthorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_author-book_id")
                            .from(BookAuthor::Table, BookAuthor::BookId)
                            .to(Book::Table, Book::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_author-author_id")
                            .from(BookAuthor::Table, BookAuthor::AuthorId)
                            .to(Author::Table, Author::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookAuthor::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BookAuthor {
    Table,
    BookId,
    AuthorId,
}
