use sea_orm::{Database, DbErr, DatabaseConnection};

// Replace with your database URL and database name
const DATABASE_URL: &str = "postgres://admin:postgres@localhost:5432";
const DB_NAME: &str = "egline";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;

    Ok(db)
}