use rocket::serde::json::Json;

use serde::{Deserialize, Serialize};

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::{book::{ActiveModel, Entity, Model}, book_rate, genre};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[derive(Debug, Serialize, Deserialize)]
struct BookWithGenresAndRates {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub cover: Vec<u8>,
    pub rating: f32,
    pub year: i32,
    pub views: i32,
    pub status: String,
    pub genres: Vec<String>,
    pub rates: usize
}

#[get("/")]
async fn get_all_books(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<BookWithGenresAndRates>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let mut books: Vec<BookWithGenresAndRates> = Vec::new();

    let query = Entity::find().all(db).await;

    match query {
        Ok(result) => {
            for result_book in result {
                let genres = result_book.find_related(genre::Entity)
                    .all(db)
                    .await
                    .unwrap()
                    .iter()
                    .map(|genre| genre.title.clone()).collect::<Vec<String>>();

                let rates = result_book.find_related(book_rate::Entity)
                    .all(db)
                    .await
                    .unwrap()
                    .len();

                let book = BookWithGenresAndRates {
                    id: result_book.id,
                    title: result_book.title.clone(),
                    description: result_book.description.clone(),
                    cover: result_book.cover.clone(),
                    rating: result_book.rating,
                    year: result_book.year,
                    views: result_book.views,
                    status: result_book.status.clone(),
                    genres,
                    rates
                };

                books.push(book);
            };

            return Ok(Json(books));
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_book_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<BookWithGenresAndRates>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let query = Entity::find_by_id(id).one(db).await;

    match query {
        Ok(Some(model)) => {
            let genres = model.find_related(genre::Entity)
                .all(db)
                .await
                .unwrap()
                .iter()
                .map(|genre| genre.title.clone()).collect::<Vec<String>>();

            let rates = model.find_related(book_rate::Entity)
                .all(db)
                .await
                .unwrap()
                .len();

            let book = BookWithGenresAndRates {
                id: model.id,
                title: model.title.clone(),
                description: model.description.clone(),
                cover: model.cover.clone(),
                rating: model.rating,
                year: model.year,
                views: model.views,
                status: model.status.clone(),
                genres,
                rates
            };

            return Ok(Json(book));
        },
        Ok(None) => Err(status::Custom(Status::InternalServerError, "No such book".to_string())),
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
        year: ActiveValue::set(book_data.year),
        views: ActiveValue::set(0),
        status: ActiveValue::set(book_data.status.clone()),
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
        year: ActiveValue::set(book_data.year),
        views: ActiveValue::set(0),
        status: ActiveValue::set(book_data.status.clone()),
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