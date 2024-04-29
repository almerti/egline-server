use sea_orm_migration::prelude::*;

use super::m20240427_222412_create_table_user::User;
use super::m20240428_222109_create_table_comment::Comment;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommentRate::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(CommentRate::Table)
                            .col(CommentRate::CommentId)
                            .col(CommentRate::UserId)
                    )
                    .col(ColumnDef::new(CommentRate::CommentId).integer().not_null())
                    .col(ColumnDef::new(CommentRate::UserId).integer().not_null())
                    .col(ColumnDef::new(CommentRate::Rate).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_rate-comment_id")
                            .from(CommentRate::Table, CommentRate::CommentId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book_rate-user_id")
                            .from(CommentRate::Table, CommentRate::UserId)
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
            .drop_table(Table::drop().table(CommentRate::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommentRate {
    Table,
    CommentId,
    UserId,
    Rate,
}
