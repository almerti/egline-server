use rocket::serde::json::Json;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::prelude::BookAuthor;
use crate::entities::book_author::{Model, ActiveModel};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_book_authors(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_authors = BookAuthor::find().all(db).await;

    match book_authors {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<book_author_data>", format="json")]
async fn create_book_author(
    db: &State<DatabaseConnection>,
    book_author_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_author:Result<Model, DbErr> = ActiveModel {
        book_id: ActiveValue::set(book_author_data.book_id.clone()),
        author_id: ActiveValue::set(book_author_data.author_id.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_author {
        Ok(_) => Ok(Json(format!("Book author was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<book_id>/<author_id>")]
async fn delete_book_author(
    db: &State<DatabaseConnection>,
    book_id: i32,
    author_id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_book_author = ActiveModel {
        book_id: ActiveValue::set(book_id),
        author_id: ActiveValue::set(author_id),
        ..Default::default()
    }.delete(db).await;

    match deleted_book_author {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_book_author_methods() -> Vec<rocket::Route> {
    routes![get_all_book_authors, create_book_author, delete_book_author]
}