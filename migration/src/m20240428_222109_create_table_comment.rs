use sea_orm_migration::prelude::*;

use super::m20240427_083430_create_table_book::Book;
use super::m20240427_222412_create_table_user::User;
use super::m20240427_223114_create_table_chapter::Chapter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::BookId).integer().not_null())
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .col(ColumnDef::new(Comment::ChapterId).integer().not_null())
                    .col(ColumnDef::new(Comment::Text).string().not_null())
                    .col(ColumnDef::new(Comment::Upvotes).integer().not_null())
                    .col(ColumnDef::new(Comment::Downvotes).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-book_id")
                            .from(Comment::Table, Comment::BookId)
                            .to(Book::Table, Book::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-user_id")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-chapter_id")
                            .from(Comment::Table, Comment::ChapterId)
                            .to(Chapter::Table, Chapter::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Comment {
    Table,
    Id,
    BookId,
    UserId,
    ChapterId,
    Text,
    Upvotes,
    Downvotes
}
