#[macro_use]
extern crate rocket;

mod setup;
use setup::set_up_db;

mod routes;
use routes::user_route;

mod entities;

use sea_orm::DatabaseConnection;

#[get("/")]
fn main_page() -> &'static str {
    "Hello, World!"
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
        .mount("/user", routes![
            user_route::get_all_users,
            user_route::create_user,
            user_route::update_user,
            user_route::delete_user,
            user_route::get_user_by_id
        ])
}
