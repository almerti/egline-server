#[macro_use]
extern crate rocket;

mod setup;
use setup::set_up_db;

mod routes;
use routes::user_route;
use routes::genre_route;
use routes::author_route;

mod entities;

use sea_orm::DatabaseConnection;

#[get("/")]
fn main_page() -> &'static str {
    "Hello!\nI am Eglien server..."
}

#[launch]
async fn rocket() -> _ {
    let db: DatabaseConnection = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![main_page])
        .mount("/user", user_route::get_all_methods())
        .mount("/genre", genre_route::get_all_methods())
        .mount("/author", author_route::get_all_methods())
}