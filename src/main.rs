#[macro_use]
extern crate rocket;

mod setup;
use setup::set_up_db;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err)
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
}