use std::fs::File;
use std::io::Read;

use chrono::NaiveDate;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::prelude::Chapter;
use crate::entities::chapter::{self, ActiveModel, Column, Model};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[get("/")]
async fn get_all_chapters(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let chapters = Chapter::find().all(db).await;

    match chapters {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/book-chapters/<id>")]
async fn get_book_chapters(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let chapters = Chapter::find().filter(Column::BookId.eq(id)).all(db).await;

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
    let chapter = Chapter::find_by_id(id).one(db).await;
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
    let is_chapter_exists = Chapter::find()
        .filter(Column::BookId.eq(chapter_data.book_id))
        .filter(Column::Number.eq(chapter_data.number))
        .one(db)
        .await;

    match is_chapter_exists {
        Ok(Some(chapter)) => Err(status::Custom(
            Status::InternalServerError,
            format!("Book {} has chapter with number {}", chapter.book_id, chapter.number)
        )),
        Ok(None) => {
            let filepath = format!("/{}/{}/", chapter_data.book_id, chapter_data.number);
            let chapter_dir = fs::create_dir(
                format!("storage{}", filepath)
            ).await;

            match chapter_dir {
                Ok(_) => {
                    let chapter:Result<Model, DbErr> = ActiveModel {
                        book_id: ActiveValue::set(chapter_data.book_id.clone()),
                        title: ActiveValue::set(chapter_data.title.clone()),
                        filepath: ActiveValue::set(filepath),
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
    let is_chapter_exists = Chapter::find()
        .filter(Column::Id.eq(id))
        .all(db)
        .await
        .unwrap();

    if is_chapter_exists[0].number != chapter_data.number {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("Book {} has chapter with number {}", is_chapter_exists[0].book_id, chapter_data.number)
        ))
    }

    let filepath = format!("/{}/{}/", chapter_data.book_id, chapter_data.number);

    let updated_chapter:Result<Model, DbErr> = ActiveModel {
        id: ActiveValue::set(id),
        book_id: ActiveValue::set(chapter_data.book_id.clone()),
        title: ActiveValue::set(chapter_data.title.clone()),
        filepath: ActiveValue::set(filepath),
        number: ActiveValue::set(chapter_data.number.clone()),
        date: ActiveValue::set(chapter_data.date),
        ..Default::default()
    }.update(db).await;

    match updated_chapter {
        Ok(_) => Ok(Json(format!("Chapter {} was successfully updated", chapter_data.title.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_chapter(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let delete_chapter = Chapter::find()
        .filter(Column::Id.eq(id))
        .all(db)
        .await
        .unwrap();

    let deleted_chapter = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_chapter {
        Ok(result) => {
            let filepath = format!("/{}/{}/", delete_chapter[0].book_id, delete_chapter[0].number);
            let chapter_dir = fs::remove_dir(
                format!("storage{}", filepath)
            ).await;

            match chapter_dir {
                Ok(_) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
                Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
            }
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/text/<chapter_id>")]
async fn get_chapter_text(
    db: &State<DatabaseConnection>,
    chapter_id: i32
) -> Result<String, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let chapter_data = Chapter::find()
        .filter(Column::Id.eq(chapter_id))
        .all(db)
        .await
        .unwrap();

    if chapter_data.len() == 0 {
        return Err(status::Custom(Status::NotFound, format!("No chapter with id {}", chapter_id)))
    }

    let filepath = format!("storage{}/text.txt", chapter_data[0].filepath);
    let file = File::options().read(true).open(filepath);

    match file {
        Ok(mut result) => {
            let mut content = String::new();
            let _ = result.read_to_string(&mut content);

            Ok(content)
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/audio/<chapter_id>")]
async fn get_chapter_audio(
    db: &State<DatabaseConnection>,
    chapter_id: i32
) -> Result<Vec<u8>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let chapter_data = Chapter::find()
        .filter(Column::Id.eq(chapter_id))
        .all(db)
        .await
        .unwrap();

    if chapter_data.len() == 0 {
        return Err(status::Custom(Status::NotFound, format!("No chapter with id {}", chapter_id)))
    }

    let filepath = format!("storage{}/audio.mp3", chapter_data[0].filepath);
    let mut file = File::options().read(true).open(filepath.clone()).expect("Can not open file");
    let metadata = fs::metadata(filepath.clone()).await.expect("Can not read metadata");
    let mut buf = vec![0; metadata.len() as usize];
    let _ = file.read(&mut buf).expect("Buffer overflow");

    Ok(buf)
}

pub fn get_all_chapter_methods() -> Vec<rocket::Route> {
    routes![
        get_all_chapters,
        get_chapter_by_id,
        create_chapter,
        update_chapter,
        delete_chapter,
        get_book_chapters,
        get_chapter_text,
        get_chapter_audio
    ]
}