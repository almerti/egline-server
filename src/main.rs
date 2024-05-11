#[macro_use]
extern crate rocket;

mod setup;
use setup::set_up_db;

mod routes;
use routes::user_route;
use routes::genre_route;
use routes::author_route;
use routes::book_route;
use routes::chapter_route;
use routes::comment_route;
use routes::book_genre_route;
use routes::book_author_route;
use routes::book_rate_route;
use routes::comment_rate_route;

mod entities;

use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[get("/")]
fn main_page() -> &'static str {
    "Hello!\nI am Egline server..."
}

#[launch]
async fn rocket() -> _ {
    let db: DatabaseConnection = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    #[derive(OpenApi)]
    #[openapi(
        info(description = "Egline API"),
        paths(user_route::get_all_users, book_route::get_all_books),
        components(schemas(entities::user::Model, book_route::BookWithGenresAndRates)),
    )]
    struct ApiDoc;

    rocket
        ::build()
        .manage(db)
        .mount("/", routes![main_page])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/user", user_route::get_all_methods())
        .mount("/genre", genre_route::get_all_methods())
        .mount("/author", author_route::get_all_methods())
        .mount("/book", book_route::get_all_methods())
        .mount("/chapter", chapter_route::get_all_chapter_methods())
        .mount("/comment", comment_route::get_all_comment_methods())
        .mount("/book-genre", book_genre_route::get_all_book_genre_methods())
        .mount("/book-author", book_author_route::get_all_book_author_methods())
        .mount("/book-rate", book_rate_route::get_all_book_rate_methods())
        .mount("/comment-rate", comment_rate_route::get_all_comment_rate_methods())
}