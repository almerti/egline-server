use std::fs::File;
use std::io::Read;
use rocket::tokio::fs;

use rocket::serde::json::Json;

use sea_orm::{ColumnTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use rocket::response::status;
use rocket::http::Status;

use rocket::State;
use utoipa::ToSchema;

use crate::entities::prelude::{Book, BookRate, Genre};
use crate::entities::book::{ActiveModel, Model, Column};
use crate::entities::{book_author, book_genre, book_rate};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookWithGenresAndRates {
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

#[utoipa::path(
    context_path = "/book",
    responses(
        (status = 200, description = "All books", body = Vec<BookWithGenresAndRates>),
        (status = 500, description = "No books", body = String)
    ),
)]
#[get("/")]
async fn get_all_books(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<BookWithGenresAndRates>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let mut books: Vec<BookWithGenresAndRates> = Vec::new();

    let query = Book::find().all(db).await;

    match query {
        Ok(result) => {
            for result_book in result {
                let genres = result_book.find_related(Genre)
                    .all(db)
                    .await
                    .unwrap()
                    .iter()
                    .map(|genre| genre.title.clone()).collect::<Vec<String>>();

                let rates = result_book.find_related(BookRate)
                    .all(db)
                    .await
                    .unwrap()
                    .len();

                let mut book = BookWithGenresAndRates {
                    id: result_book.id,
                    title: result_book.title.clone(),
                    description: result_book.description.clone(),
                    cover: Vec::new(),
                    rating: result_book.rating,
                    year: result_book.year,
                    views: result_book.views,
                    status: result_book.status.clone(),
                    genres,
                    rates
                };

                let filepath = format!("storage/{}/cover.png", result_book.id);
                let file = File::options().read(true).open(filepath.clone());

                match file {
                    Ok(mut res) => {
                        let metadata = fs::metadata(filepath.clone()).await.expect("Can not read metadata");
                        let mut buf = vec![0; metadata.len() as usize];
                        let _ = res.read(&mut buf).expect("Buffer overflow");

                        book.cover = buf;
                    }
                    Err(_) => {
                        book.cover = Vec::new();
                    }
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
    let query = Book::find_by_id(id).one(db).await;

    match query {
        Ok(Some(model)) => {
            let genres = model.find_related(Genre)
                .all(db)
                .await
                .unwrap()
                .iter()
                .map(|genre| genre.title.clone()).collect::<Vec<String>>();

            let rates = model.find_related(BookRate)
                .all(db)
                .await
                .unwrap()
                .len();
            
            let mut book = BookWithGenresAndRates {
                id: model.id,
                title: model.title.clone(),
                description: model.description.clone(),
                cover: Vec::new(),
                rating: model.rating,
                year: model.year,
                views: model.views,
                status: model.status.clone(),
                genres,
                rates
            };

            let filepath = format!("storage/{}/cover.png", model.id);
            let file = File::options().read(true).open(filepath.clone());
            
            match file {
                Ok(mut res) => {
                    let metadata = fs::metadata(filepath.clone()).await.expect("Can not read metadata");
                    let mut buf = vec![0; metadata.len() as usize];
                    let _ = res.read(&mut buf).expect("Buffer overflow");

                    book.cover = buf;
                }
                Err(_) => {
                    book.cover = Vec::new();
                }
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

#[get("/get-ids")]
async fn get_ids(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<i32>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let books = Book::find().order_by_asc(Column::Rating).all(db).await.expect("Error");
    let ids = books.iter().map(|book| book.id).collect::<Vec<i32>>();

    Ok(Json(ids))
}

#[post("/genre", data="<book_genre_data>", format="json")]
pub async fn add_genre_to_book(
    db: &State<DatabaseConnection>,
    book_genre_data: Json<book_genre::Model>
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_genre = book_genre::ActiveModel {
        book_id: ActiveValue::set(book_genre_data.book_id.clone()),
        genre_id: ActiveValue::set(book_genre_data.genre_id.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_genre {
        Ok(_) => Ok(Json(format!("Book genre was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/genre/<book_id>/<genre_id>")]
pub async fn delete_genre_from_book(
    db: &State<DatabaseConnection>,
    book_id: i32,
    genre_id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_genre = book_genre::ActiveModel {
        book_id: ActiveValue::set(book_id.clone()),
        genre_id: ActiveValue::set(genre_id.clone()),
        ..Default::default()
    }.delete(db).await;

    match book_genre {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/author", data="<book_author_data>", format="json")]
pub async fn add_author_to_book(
    db: &State<DatabaseConnection>,
    book_author_data: Json<book_author::Model>
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_author = book_author::ActiveModel {
        book_id: ActiveValue::set(book_author_data.book_id.clone()),
        author_id: ActiveValue::set(book_author_data.author_id.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_author {
        Ok(_) => Ok(Json(format!("Book author was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/author/<book_id>/<author_id>")]
pub async fn delete_author_from_book(
    db: &State<DatabaseConnection>,
    book_id: i32,
    author_id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_author = book_author::ActiveModel {
        book_id: ActiveValue::set(book_id.clone()),
        author_id: ActiveValue::set(author_id.clone()),
        ..Default::default()
    }.delete(db).await;

    match book_author {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/rate", data="<book_rate_data>", format="json")]
pub async fn add_rate_to_book(
    db: &State<DatabaseConnection>,
    book_rate_data: Json<book_rate::Model>
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if book_rate_data.rate < 1 || book_rate_data.rate > 5 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let book_rate = book_rate::ActiveModel {
        book_id: ActiveValue::set(book_rate_data.book_id.clone()),
        user_id: ActiveValue::set(book_rate_data.user_id.clone()),
        rate: ActiveValue::set(book_rate_data.rate.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_rate {
        Ok(result) => {
            let book_rates = BookRate::find()
                .filter(book_rate::Column::BookId.eq(result.book_id))
                .all(db)
                .await
                .unwrap()
                .iter()
                .map(|rate| rate.rate.clone()).collect::<Vec<i32>>();

            let new_rating = book_rates.iter().sum::<i32>() as f32 / book_rates.len() as f32;

            let upated_book = ActiveModel {
                id: ActiveValue::set(result.book_id),
                rating: ActiveValue::set(new_rating),
                ..Default::default()
            }.update(db).await;

            match upated_book {
                Ok(_) => Ok(Json(format!("Book rate was successfully created"))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/rate", data="<book_rate_data>", format="json")]
pub async fn update_rate_to_book(
    db: &State<DatabaseConnection>,
    book_rate_data: Json<book_rate::Model>
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if book_rate_data.rate < 1 || book_rate_data.rate > 5 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let book_rate = book_rate::ActiveModel {
        book_id: ActiveValue::set(book_rate_data.book_id.clone()),
        user_id: ActiveValue::set(book_rate_data.user_id.clone()),
        rate: ActiveValue::set(book_rate_data.rate.clone()),
        ..Default::default()
    }.update(db).await;

    match book_rate {
        Ok(result) => {
            let book_rates = BookRate::find()
                .filter(book_rate::Column::BookId.eq(result.book_id))
                .all(db)
                .await
                .unwrap()
                .iter()
                .map(|rate| rate.rate.clone()).collect::<Vec<i32>>();

            let new_rating = book_rates.iter().sum::<i32>() as f32 / book_rates.len() as f32;

            let upated_book = ActiveModel {
                id: ActiveValue::set(result.book_id),
                rating: ActiveValue::set(new_rating),
                ..Default::default()
            }.update(db).await;

            match upated_book {
                Ok(_) => Ok(Json(format!("Book rate was successfully created"))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/rate/<book_id>/<user_id>")]
pub async fn delete_rate_from_book(
    db: &State<DatabaseConnection>,
    book_id: i32,
    user_id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_rate = book_rate::ActiveModel {
        book_id: ActiveValue::set(book_id.clone()),
        user_id: ActiveValue::set(user_id.clone()),
        ..Default::default()
    }.delete(db).await;

    match book_rate {
        Ok(_) => {
            let book_rates = BookRate::find()
                .filter(book_rate::Column::BookId.eq(book_id))
                .all(db)
                .await
                .unwrap()
                .iter()
                .map(|rate| rate.rate.clone()).collect::<Vec<i32>>();

            let new_rating = book_rates.iter().sum::<i32>() as f32 / book_rates.len() as f32;

            let upated_book = ActiveModel {
                id: ActiveValue::set(book_id),
                rating: ActiveValue::set(new_rating),
                ..Default::default()
            }.update(db).await;

            match upated_book {
                Ok(_) => Ok(Json(format!("Book rate was successfully deleted"))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![
        get_all_books,
        get_book_by_id,
        create_book,
        update_book,
        delete_book,
        add_genre_to_book,
        delete_genre_from_book,
        add_author_to_book,
        delete_author_from_book,
        add_rate_to_book,
        update_rate_to_book,
        delete_rate_from_book,
        get_ids
    ]
}