use rocket::serde::json::Json;
use rocket::State;

use rocket::response::status;
use rocket::http::Status;

use crate::entities::comment_rate::{Entity, Model, ActiveModel};
use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn get_all_comment_rates(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let comment_rates = Entity::find().all(db).await;

    match comment_rates {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<comment_rate_data>", format="json")]
async fn create_comment_rate(
    db: &State<DatabaseConnection>,
    comment_rate_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if comment_rate_data.rate != -1 || comment_rate_data.rate != 1 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let comment_rate:Result<Model, DbErr> = ActiveModel {
        comment_id: ActiveValue::set(comment_rate_data.comment_id.clone()),
        user_id: ActiveValue::set(comment_rate_data.user_id.clone()),
        rate: ActiveValue::set(comment_rate_data.rate.clone()),
        ..Default::default()
    }.insert(db).await;

    match comment_rate {
        Ok(_) => Ok(Json(format!("Comment rate was successfully created"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/", data="<comment_rate_data>", format="json")]
async fn update_comment_rate(
    db: &State<DatabaseConnection>,
    comment_rate_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    if comment_rate_data.rate < 1 || comment_rate_data.rate > 5 {
        return Err(status::Custom(
            Status::InternalServerError,
            "Saving rate error: Invalid rate value".to_string()
        ))
    }

    let comment_rate:Result<Model, DbErr> = ActiveModel {
        comment_id: ActiveValue::set(comment_rate_data.comment_id.clone()),
        user_id: ActiveValue::set(comment_rate_data.user_id.clone()),
        rate: ActiveValue::set(comment_rate_data.rate.clone()),
        ..Default::default()
    }.update(db).await;

    match comment_rate {
        Ok(_) => Ok(Json(format!("Comment rate was successfully updated"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<comment_id>/<user_id>")]
async fn delete_comment_rate(
    db: &State<DatabaseConnection>,
    comment_id: i32,
    user_id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_comment_rate = ActiveModel {
        comment_id: ActiveValue::set(comment_id),
        user_id: ActiveValue::set(user_id),
        ..Default::default()
    }.delete(db).await;

    match deleted_comment_rate {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_comment_rate_methods() -> Vec<rocket::Route> {
    routes![get_all_comment_rates, create_comment_rate, update_comment_rate, delete_comment_rate]
}