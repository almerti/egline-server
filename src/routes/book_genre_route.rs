use rocket::serde::json::Json;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::prelude::BookGenre;
use crate::entities::book_genre::{Model, ActiveModel};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_book_genres(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_genres = BookGenre::find().all(db).await;

    match book_genres {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<book_genre_data>", format="json")]
async fn create_book_genre(
    db: &State<DatabaseConnection>,
    book_genre_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_genre:Result<Model, DbErr> = ActiveModel {
        book_id: ActiveValue::set(book_genre_data.book_id.clone()),
        genre_id: ActiveValue::set(book_genre_data.genre_id.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_genre {
        Ok(_) => Ok(Json(format!("Book genre was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<book_id>/<genre_id>")]
async fn delete_book_genre(
    db: &State<DatabaseConnection>,
    book_id: i32,
    genre_id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_book_genre = ActiveModel {
        book_id: ActiveValue::set(book_id),
        genre_id: ActiveValue::set(genre_id),
        ..Default::default()
    }.delete(db).await;

    match deleted_book_genre {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_book_genre_methods() -> Vec<rocket::Route> {
    routes![get_all_book_genres, create_book_genre, delete_book_genre]
}