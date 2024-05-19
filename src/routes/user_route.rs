use rocket::serde::json::{Json, serde_json};
use serde::{Deserialize, Serialize};

use serde_json::json;
use json_value_remove::Remove;

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::user::{Model, ActiveModel, Column};
use crate::entities::prelude::{User, Book};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};

use sha256::digest;

#[derive(Debug, Serialize, Deserialize)]
struct UserAuthModel {
    email: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UserEditModel {
    email: String,
    display_name: String,
    password: String,
    new_password: String,
    avatar: Vec<u8>
}

#[derive(Debug, Serialize, Deserialize)]
struct UserWithoutPassword {
    id: i32,
    email: String,
    display_name: String,
    avatar: Vec<u8>,
    saved_books: serde_json::Value,
}

#[utoipa::path(
    context_path = "/user",
    responses(
        (status = 200, description = "All users", body = Vec<Model>),
        (status = 500, description = "No users", body = String)
    ),
)]
#[get("/")]
async fn get_all_users(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let users = User::find().all(db).await;

    match users {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_user_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let user = User::find_by_id(id).one(db).await;

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => {
            let empty_user = Model {
                id: -1,
                display_name: String::new(),
                email: String::new(),
                password: String::new(),
                avatar: Vec::new(),
                saved_books: json!(""),
            };
            Ok(Json(empty_user))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<user_data>", format="json")]
async fn create_user(
    db: &State<DatabaseConnection>,
    user_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let hashed_password: String = digest(user_data.password.clone());

    let user:Result<Model, DbErr> = ActiveModel {
        display_name: ActiveValue::set(user_data.display_name.clone()),
        email: ActiveValue::set(user_data.email.clone().to_lowercase()),
        password: ActiveValue::set(hashed_password),
        avatar: ActiveValue::set(user_data.avatar.clone()),
        saved_books: ActiveValue::set(json!(user_data.saved_books.clone())),
        ..Default::default()
    }.insert(db).await;

    match user {
        Ok(_) => Ok(Json(format!("User {} was successfully created", user_data.display_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<user_data>", format="json")]
async fn update_user(
    db: &State<DatabaseConnection>,
    user_data: Json<Model>,
    id: i32,
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let user = User::find_by_id(id).one(db).await.unwrap().unwrap();

    let hashed_password = if user_data.password.is_empty() {
        user.password
    } else {
        digest(user_data.password.clone())
    };

    let updated_user = ActiveModel {
        id: ActiveValue::set(id),
        display_name: ActiveValue::set(user_data.display_name.clone()),
        email: ActiveValue::set(user_data.email.clone().to_lowercase()),
        password: ActiveValue::set(hashed_password),
        avatar: ActiveValue::set(user_data.avatar.clone()),
        saved_books: ActiveValue::set(json!(user_data.saved_books.clone())),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_user(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_user = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_user {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveBook {
    user_id: i32,
    book_id: i32,
    tab_name: String
}

#[post("/save-book", data="<save_book_data>", format="json")]
async fn add_book_to_tab(
    db: &State<DatabaseConnection>,
    save_book_data: Json<SaveBook>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let user = User::find_by_id(save_book_data.user_id).all(db).await.unwrap();
    let book = Book::find_by_id(save_book_data.book_id).all(db).await.unwrap();

    if user.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No user with id {}", save_book_data.user_id)))
    }

    if book.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No book with id {}", save_book_data.book_id)))
    }

    let tab_name = save_book_data.tab_name.clone();

    let mut saved_books = user[0].saved_books.clone();
    let tab_array = saved_books[tab_name.clone()].as_array_mut();

    match tab_array {
        Some(result) => {
            if !result.contains(&json!(save_book_data.book_id)) {
                result.push(json!(save_book_data.book_id))
            }
        }
        None => {
            saved_books[tab_name.clone()] = json!([save_book_data.book_id])
        }
    }
    
    let updated_user = ActiveModel {
        id: ActiveValue::set(save_book_data.user_id),
        saved_books: ActiveValue::set(saved_books),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(_) => Ok(Json(format!("Tab {} was updated", tab_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/delete-book", data="<save_book_data>", format="json")]
async fn delete_book_from_tab(
    db: &State<DatabaseConnection>,
    save_book_data: Json<SaveBook>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let user = User::find_by_id(save_book_data.user_id).all(db).await.unwrap();
    let book = Book::find_by_id(save_book_data.book_id).all(db).await.unwrap();

    if user.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No user with id {}", save_book_data.user_id)))
    }

    if book.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No book with id {}", save_book_data.book_id)))
    }

    let tab_name = save_book_data.tab_name.clone();

    let mut saved_books = user[0].saved_books.clone();
    let tab_array = saved_books[tab_name.clone()].as_array_mut();

    match tab_array {
        Some(result) => {
            if result.contains(&json!(save_book_data.book_id)) {
                let index = result.iter().position(|x| *x == json!(save_book_data.book_id)).unwrap();
                result.remove(index);
            }
        }
        None => {
            return Err(status::Custom(
                Status::InternalServerError,
                format!("User with {} id does not have tab {}", save_book_data.user_id, save_book_data.tab_name)
            ));
        }
    }
    
    let updated_user = ActiveModel {
        id: ActiveValue::set(save_book_data.user_id),
        saved_books: ActiveValue::set(saved_books),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(_) => Ok(Json(format!("Tab {} was updated", tab_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/tab/<user_id>/<tab_name>")]
async fn add_tab_to_user(
    db: &State<DatabaseConnection>,
    user_id: i32,
    tab_name: String
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let user = User::find_by_id(user_id.clone()).all(db).await.unwrap();

    if user.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No user with id {}", user_id)))
    }

    let mut saved_books = user[0].saved_books.clone();
    let tab_array = saved_books[tab_name.clone()].as_array_mut();

    match tab_array {
        Some(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                format!("User with {} id already has {} tab", user_id, tab_name)
            ));
        }
        None => {
            saved_books[tab_name.clone()] = json!([]);
        }
    }

    let updated_user = ActiveModel {
        id: ActiveValue::set(user_id),
        saved_books: ActiveValue::set(saved_books),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(_) => Ok(Json(format!("Tab {} was successfully added to user with {} id", tab_name.clone(), user_id.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/tab/<user_id>/<tab_name>")]
async fn delete_user_tab(
    db: &State<DatabaseConnection>,
    user_id: i32,
    tab_name: String
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let user = User::find_by_id(user_id.clone()).all(db).await.unwrap();

    if user.len() == 0 {
        return Err(status::Custom(Status::InternalServerError, format!("No user with id {}", user_id)))
    }

    let mut saved_books = user[0].saved_books.clone();
    let tab_array = saved_books[tab_name.clone()].as_array_mut();

    match tab_array {
        Some(_) => {
            let _ = saved_books.remove(format!("/{}", tab_name.clone()).as_str());
        }
        None => {
            return Err(status::Custom(
                Status::InternalServerError,
                format!("User with id {} doesn't have {} tab", user_id, tab_name)
            ));
        }
    }

    let updated_user = ActiveModel {
        id: ActiveValue::set(user_id),
        saved_books: ActiveValue::set(saved_books),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(_) => Ok(Json(format!("Tab {} was successfully added to user with {} id", tab_name.clone(), user_id.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/login", data = "<user_auth_data>", format = "json")]
async fn login_user(
    db: &State<DatabaseConnection>,
    user_auth_data: Json<UserAuthModel>
) -> Result<Json<UserWithoutPassword>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let hashed_password = digest(user_auth_data.password.clone());

    let user = User::find()
        .filter(Column::Email.eq(user_auth_data.email.clone()))
        .filter(Column::Password.eq(hashed_password))
        .one(db)
        .await;

    match user {
        Ok(Some(result)) => {
            let user_wo_password = UserWithoutPassword {
                id: result.id,
                email: result.email,
                display_name: result.display_name,
                avatar: result.avatar,
                saved_books: result.saved_books
            };

            Ok(Json(user_wo_password))
        },
        Ok(None) => Err(status::Custom(Status::BadRequest, format!("Email or passwrod are not valid"))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/edit/<id>", data = "<user_edit_data>", format = "json")]
async fn edit_user(
    db: &State<DatabaseConnection>,
    id: i32,
    user_edit_data: Json<UserEditModel>
) -> Result<Json<UserWithoutPassword>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let user = User::find_by_id(id).all(db).await.unwrap();

    if user.len() == 0 {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("No such user with id {}", id)
        ))
    }

    let hashed_old_password = digest(user_edit_data.password.clone());
    let hashed_new_password = digest(user_edit_data.new_password.clone());

    if user[0].password.clone() != hashed_old_password.clone() && user_edit_data.password.clone().len() != 0 {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("Passwords missmatching")
        ))
    }

    let updated_user = ActiveModel {
        id: ActiveValue::set(id),
        email: ActiveValue::set(user_edit_data.email.clone()),
        display_name: ActiveValue::set(user_edit_data.display_name.clone()),
        password: ActiveValue::set(
        if user_edit_data.new_password.clone().len() == 0 {
            user[0].password.clone()
        } else {
            hashed_new_password.clone()
        }),
        avatar: ActiveValue::set(user_edit_data.avatar.clone()),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(result) => {
            let user_wo_password = UserWithoutPassword {
                id: result.id,
                email: result.email,
                display_name: result.display_name,
                avatar: result.avatar,
                saved_books: result.saved_books
            };

            Ok(Json(user_wo_password))
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }

}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![
        get_all_users,
        get_user_by_id,
        create_user,
        update_user,
        delete_user,
        add_tab_to_user,
        delete_user_tab,
        add_book_to_tab,
        delete_book_from_tab,
        login_user,
        edit_user
    ]
}