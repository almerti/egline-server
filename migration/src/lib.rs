pub use sea_orm_migration::prelude::*;

mod m20240427_083430_create_table_book;
mod m20240427_164602_create_table_genre;
mod m20240427_164712_create_table_author;
mod m20240427_222412_create_table_user;
mod m20240427_223114_create_table_chapter;
mod m20240428_214711_create_table_bookgenre;
mod m20240428_214720_create_table_bookauthor;
mod m20240428_222109_create_table_comment;
mod m20240428_225721_create_table_book_rate;
mod m20240428_230452_create_table_comment_rate;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240427_083430_create_table_book::Migration),
            Box::new(m20240427_164602_create_table_genre::Migration),
            Box::new(m20240427_164712_create_table_author::Migration),
            Box::new(m20240427_222412_create_table_user::Migration),
            Box::new(m20240427_223114_create_table_chapter::Migration),
            Box::new(m20240428_214711_create_table_bookgenre::Migration),
            Box::new(m20240428_214720_create_table_bookauthor::Migration),
            Box::new(m20240428_222109_create_table_comment::Migration),
            Box::new(m20240428_225721_create_table_book_rate::Migration),
            Box::new(m20240428_230452_create_table_comment_rate::Migration),
        ]
    }
}
