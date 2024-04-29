use sea_orm::*;

// Replace with your database URL and database name
const DATABASE_URL: &str = "postgres://admin:postgres@localhost:5432";
const DB_NAME: &str = "egline";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        
        DbBackend::Postgres => {
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }

        DbBackend::Sqlite => db,
    };

    Ok(db)
}