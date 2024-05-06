use chrono::NaiveDate;
use rocket::serde::json::Json;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::chapter::{self, ActiveModel, Entity, Model};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[get("/")]
async fn get_all_chapters(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let chapters = Entity::find().all(db).await;

    match chapters {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_chapter_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let chapter = Entity::find_by_id(id).one(db).await;
    let null_date = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();

    match chapter {
        Ok(Some(chapter)) => Ok(Json(chapter)),
        Ok(None) => {
            let empty_chapter = Model {
                id: -1,
                book_id: -1,
                title: String::new(),
                filepath: String::new(),
                number: -1,
                date: null_date
            };
            Ok(Json(empty_chapter))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<chapter_data>", format="json")]
async fn create_chapter(
    db: &State<DatabaseConnection>,
    chapter_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let is_chapter_exists = Entity::find()
        .filter(chapter::Column::BookId.eq(chapter_data.book_id))
        .filter(chapter::Column::Number.eq(chapter_data.number))
        .one(db)
        .await;

    match is_chapter_exists {
        Ok(Some(chapter)) => Err(status::Custom(
            Status::InternalServerError,
            format!("Book {} has chapter with number {}", chapter.book_id, chapter.number)
        )),
        Ok(None) => {
            let chapter:Result<Model, DbErr> = ActiveModel {
                book_id: ActiveValue::set(chapter_data.book_id.clone()),
                title: ActiveValue::set(chapter_data.title.clone()),
                filepath: ActiveValue::set(chapter_data.filepath.clone()),
                number: ActiveValue::set(chapter_data.number.clone()),
                date: ActiveValue::set(chapter_data.date),
                ..Default::default()
            }.insert(db).await;

            match chapter {
                Ok(_) => Ok(Json(format!("Chapter {} was successfully created", chapter_data.title.clone()))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<chapter_data>", format="json")]
async fn update_chapter(
    db: &State<DatabaseConnection>,
    chapter_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let is_chapter_exists = Entity::find()
        .filter(chapter::Column::BookId.eq(chapter_data.book_id))
        .filter(chapter::Column::Number.eq(chapter_data.number))
        .one(db)
        .await;

    match is_chapter_exists {
        Ok(Some(chapter)) => Err(status::Custom(
            Status::InternalServerError,
            format!("Book {} has chapter with number {}", chapter.book_id, chapter.number)
        )),
        Ok(None) => {
            let updated_chapter:Result<Model, DbErr> = ActiveModel {
                id: ActiveValue::set(id),
                book_id: ActiveValue::set(chapter_data.book_id.clone()),
                title: ActiveValue::set(chapter_data.title.clone()),
                filepath: ActiveValue::set(chapter_data.filepath.clone()),
                number: ActiveValue::set(chapter_data.number.clone()),
                date: ActiveValue::set(chapter_data.date),
                ..Default::default()
            }.insert(db).await;

            match updated_chapter {
                Ok(_) => Ok(Json(format!("Chapter {} was successfully updated", chapter_data.title.clone()))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_chapter(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_chapter = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_chapter {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_chapter_methods() -> Vec<rocket::Route> {
    routes![get_all_chapters, get_chapter_by_id, create_chapter, update_chapter, delete_chapter]
}