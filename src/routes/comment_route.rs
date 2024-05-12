use rocket::serde::json::Json;
use rocket::response::status;

use rocket::http::Status;
use rocket::State;
use sea_orm::{ColumnTrait, QueryFilter};

use crate::entities::prelude::{Comment, Chapter};
use crate::entities::comment::{ActiveModel, Model};
use crate::entities::chapter;

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_comments(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let comments = Comment::find().all(db).await;

    match comments {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_comment_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let comment = Comment::find_by_id(id).one(db).await;

    match comment {
        Ok(Some(comment)) => Ok(Json(comment)),
        Ok(None) => {
            let empty_comment = Model {
                id: -1,
                book_id: -1,
                user_id: -1,
                chapter_id: -1,
                text: String::new(),
                upvotes: 0,
                downvotes: 0,
            };
            Ok(Json(empty_comment))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<comment_data>", format="json")]
async fn create_comment(
    db: &State<DatabaseConnection>,
    comment_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let chapter = Chapter::find_by_id(comment_data.chapter_id)
        .filter(chapter::Column::BookId.eq(comment_data.book_id))
        .all(db)
        .await
        .unwrap();
    
    if chapter.is_empty() {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("No such chapter with id {} and book_id {}", comment_data.chapter_id, comment_data.book_id)
        ));
    }
    
    let comment:Result<Model, DbErr> = ActiveModel {
        book_id: ActiveValue::set(comment_data.book_id.clone()),
        user_id: ActiveValue::set(comment_data.user_id.clone()),
        chapter_id: ActiveValue::set(comment_data.chapter_id.clone()),
        text: ActiveValue::set(comment_data.text.clone()),
        upvotes: ActiveValue::set(comment_data.upvotes.clone()),
        downvotes: ActiveValue::set(comment_data.downvotes.clone()),
        ..Default::default()
    }.insert(db).await;

    match comment {
        Ok(_) => Ok(Json(format!("Comment was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<comment_data>", format="json")]
async fn update_comment(
    db: &State<DatabaseConnection>,
    comment_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let chapter = Chapter::find_by_id(comment_data.chapter_id)
        .filter(chapter::Column::BookId.eq(comment_data.book_id))
        .all(db)
        .await
        .unwrap();
    
    if chapter.is_empty() {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("No such chapter with id {} and book_id {}", comment_data.chapter_id, comment_data.book_id)
        ));
    }

    let updated_comment = ActiveModel {
        id: ActiveValue::set(id),
        book_id: ActiveValue::set(comment_data.book_id.clone()),
        user_id: ActiveValue::set(comment_data.user_id.clone()),
        chapter_id: ActiveValue::set(comment_data.chapter_id.clone()),
        text: ActiveValue::set(comment_data.text.clone()),
        upvotes: ActiveValue::set(comment_data.upvotes.clone()),
        downvotes: ActiveValue::set(comment_data.downvotes.clone()),
        ..Default::default()
    }.update(db).await;

    match updated_comment {
        Ok(result) => Ok(Json(format!("Comment {} was successfully updated", result.id.to_string()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_comment(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_comment = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_comment {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_comment_methods() -> Vec<rocket::Route> {
    routes![get_all_comments, get_comment_by_id, create_comment, update_comment, delete_comment]
}