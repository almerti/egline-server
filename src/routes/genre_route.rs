use rocket::serde::json::Json;

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::{genre::Entity, genre::Model, genre::ActiveModel};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_genres(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let genres = Entity::find().all(db).await;

    match genres {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_genre_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let genre = Entity::find_by_id(id).one(db).await;

    match genre {
        Ok(Some(genre)) => Ok(Json(genre)),
        Ok(None) => {
            let empty_genre = Model {
                id: -1,
                title: String::new(),
            };
            Ok(Json(empty_genre))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<genre_data>", format="json")]
async fn create_genre(
    db: &State<DatabaseConnection>,
    genre_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let mut genre_title = genre_data.title.clone().to_lowercase();
    let title = genre_title.remove(0).to_uppercase().to_string() + &genre_title;

    let genre:Result<Model, DbErr> = ActiveModel {
        title: ActiveValue::set(title),
        ..Default::default()
    }.insert(db).await;

    match genre {
        Ok(_) => Ok(Json(format!("Genre {} was successfully created", genre_data.title.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<genre_data>", format="json")]
async fn update_genre(
    db: &State<DatabaseConnection>,
    genre_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let mut genre_title = genre_data.title.clone().to_lowercase();
    let title = genre_title.remove(0).to_uppercase().to_string() + &genre_title;

    let updated_genre = ActiveModel {
        id: ActiveValue::set(id),
        title: ActiveValue::set(title),
        ..Default::default()
    }.update(db).await;

    match updated_genre {
        Ok(result) => Ok(Json(format!("Genre {} was successfully updated", result.title.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_genre(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_genre = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_genre {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![get_all_genres, get_genre_by_id, create_genre, update_genre, delete_genre]
}