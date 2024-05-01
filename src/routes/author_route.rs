use rocket::serde::json::Json;

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::{author::Entity, author::Model, author::ActiveModel};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_authors(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let authors = Entity::find().all(db).await;

    match authors {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_author_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let author = Entity::find_by_id(id).one(db).await;

    match author {
        Ok(Some(author)) => Ok(Json(author)),
        Ok(None) => {
            let empty_author = Model {
                id: -1,
                first_name: String::new(),
                last_name: String::new(),
                biography: String::new(),
                rating: 0.0,
                avatar: Vec::new(),
            };
            Ok(Json(empty_author))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<author_data>", format="json")]
async fn create_author(
    db: &State<DatabaseConnection>,
    author_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let author:Result<Model, DbErr> = ActiveModel {
        first_name: ActiveValue::set(author_data.first_name.clone()),
        last_name: ActiveValue::set(author_data.last_name.clone()),
        biography: ActiveValue::set(author_data.biography.clone()),
        rating: ActiveValue::set(0.0),
        avatar: ActiveValue::set(author_data.avatar.clone()),
        ..Default::default()
    }.insert(db).await;

    match author {
        Ok(_) => Ok(Json(format!("Author {} {} was successfully created", author_data.first_name.clone(), author_data.last_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<author_data>", format="json")]
async fn update_author(
    db: &State<DatabaseConnection>,
    author_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let updated_author = ActiveModel {
        id: ActiveValue::set(id),
        first_name: ActiveValue::set(author_data.first_name.clone()),
        last_name: ActiveValue::set(author_data.last_name.clone()),
        biography: ActiveValue::set(author_data.biography.clone()),
        rating: ActiveValue::set(author_data.rating),
        avatar: ActiveValue::set(author_data.avatar.clone()),
        ..Default::default()
    }.update(db).await;

    match updated_author {
        Ok(result) => Ok(Json(format!("Author {} {} was successfully updated", result.first_name.clone(), result.last_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_author(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_author = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_author {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![get_all_authors, get_author_by_id, create_author, update_author, delete_author]
}