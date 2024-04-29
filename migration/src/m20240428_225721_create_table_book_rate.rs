use sea_orm_migration::prelude::*;

use super::m20240427_083430_create_table_book::Book;
use super::m20240427_222412_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookRate::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(BookRate::Table)
                            .col(BookRate::BookId)
                            .col(BookRate::UserId)
                    )
                    .col(ColumnDef::new(BookRate::BookId).integer().not_null())
                    .col(ColumnDef::new(BookRate::UserId).integer().not_null())
                    .col(ColumnDef::new(BookRate::Rate).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_rate-book_id")
                            .from(BookRate::Table, BookRate::BookId)
                            .to(Book::Table, Book::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_rate-user_id")
                            .from(BookRate::Table, BookRate::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookRate::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BookRate {
    Table,
    BookId,
    UserId,
    Rate,
}
