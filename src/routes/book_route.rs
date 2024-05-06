use rocket::serde::json::Json;

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::{book::Entity, book::Model, book::ActiveModel, genre, book_genre};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_books(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let books = Entity::find().all(db).await;

    match books {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_book_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let book = Entity::find_by_id(id).one(db).await;

    match book {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => {
            let empty_book = Model {
                id: -1,
                title: String::new(),
                description: String::new(),
                cover: Vec::new(),
                rating: 0.0,
            };
            Ok(Json(empty_book))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<book_data>", format="json")]
async fn create_book(
    db: &State<DatabaseConnection>,
    book_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book:Result<Model, DbErr> = ActiveModel {
        title: ActiveValue::set(book_data.title.clone()),
        description: ActiveValue::set(book_data.description.clone()),
        cover: ActiveValue::set(book_data.cover.clone()),
        rating: ActiveValue::set(0.0),
        ..Default::default()
    }.insert(db).await;

    match book {
        Ok(_) => Ok(Json(format!("Book {} was successfully created", book_data.title.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<book_data>", format="json")]
async fn update_book(
    db: &State<DatabaseConnection>,
    book_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let updated_book = ActiveModel {
        id: ActiveValue::set(id),
        title: ActiveValue::set(book_data.title.clone()),
        description: ActiveValue::set(book_data.description.clone()),
        cover: ActiveValue::set(book_data.cover.clone()),
        rating: ActiveValue::set(book_data.rating),
        ..Default::default()
    }.update(db).await;

    match updated_book {
        Ok(result) => Ok(Json(format!("Book {} was successfully updated", result.title.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_book(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_book = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_book {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![get_all_books, get_book_by_id, create_book, update_book, delete_book]
}