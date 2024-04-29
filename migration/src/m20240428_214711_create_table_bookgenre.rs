use sea_orm_migration::prelude::*;

use super::m20240427_083430_create_table_book::Book;
use super::m20240427_164602_create_table_genre::Genre;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookGenre::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(BookGenre::Table)
                            .col(BookGenre::BookId)
                            .col(BookGenre::GenreId)
                    )
                    .col(ColumnDef::new(BookGenre::BookId).integer().not_null())
                    .col(ColumnDef::new(BookGenre::GenreId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_genre-book_id")
                            .from(BookGenre::Table, BookGenre::BookId)
                            .to(Book::Table, Book::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_genre-genre_id")
                            .from(BookGenre::Table, BookGenre::GenreId)
                            .to(Genre::Table, Genre::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookGenre::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BookGenre {
    Table,
    BookId,
    GenreId
}
