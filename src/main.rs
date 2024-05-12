#[macro_use]
extern crate rocket;

mod setup;
use rocket::response::content::RawHtml;
use setup::set_up_db;

mod routes;
use routes::user_route;
use routes::genre_route;
use routes::author_route;
use routes::book_route;
use routes::chapter_route;
use routes::comment_route;
use routes::comment_rate_route;

mod entities;

use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml("Hello!\nI am Egline server...\n<a href='swagger-ui/'>swagger</a>")
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
        .mount("/", routes![index])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .mount("/api/v1/user", user_route::get_all_methods())
        .mount("/api/v1/genre", genre_route::get_all_methods())
        .mount("/api/v1/author", author_route::get_all_methods())
        .mount("/api/v1/book", book_route::get_all_methods())
        .mount("/api/v1/chapter", chapter_route::get_all_chapter_methods())
        .mount("/api/v1/comment", comment_route::get_all_comment_methods())
        .mount("/api/v1/comment-rate", comment_rate_route::get_all_comment_rate_methods())
}