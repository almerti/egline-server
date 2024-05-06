use rocket::serde::json::Json;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::book_rate::{Entity, Model, ActiveModel};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_book_rates(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let book_rates = Entity::find().all(db).await;

    match book_rates {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<book_rate_data>", format="json")]
async fn create_book_rate(
    db: &State<DatabaseConnection>,
    book_rate_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if book_rate_data.rate < 1 || book_rate_data.rate > 5 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let book_rate:Result<Model, DbErr> = ActiveModel {
        book_id: ActiveValue::set(book_rate_data.book_id.clone()),
        user_id: ActiveValue::set(book_rate_data.user_id.clone()),
        rate: ActiveValue::set(book_rate_data.rate.clone()),
        ..Default::default()
    }.insert(db).await;

    match book_rate {
        Ok(_) => Ok(Json(format!("Book rate was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/", data="<book_rate_data>", format="json")]
async fn update_book_rate(
    db: &State<DatabaseConnection>,
    book_rate_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if book_rate_data.rate < 1 || book_rate_data.rate > 5 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let book_rate:Result<Model, DbErr> = ActiveModel {
        book_id: ActiveValue::set(book_rate_data.book_id.clone()),
        user_id: ActiveValue::set(book_rate_data.user_id.clone()),
        rate: ActiveValue::set(book_rate_data.rate.clone()),
        ..Default::default()
    }.update(db).await;

    match book_rate {
        Ok(_) => Ok(Json(format!("Book rate was successfully updated"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<book_id>/<user_id>")]
async fn delete_book_rate(
    db: &State<DatabaseConnection>,
    book_id: i32,
    user_id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_book_rate = ActiveModel {
        book_id: ActiveValue::set(book_id),
        user_id: ActiveValue::set(user_id),
        ..Default::default()
    }.delete(db).await;

    match deleted_book_rate {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_book_rate_methods() -> Vec<rocket::Route> {
    routes![get_all_book_rates, create_book_rate, update_book_rate, delete_book_rate]
}