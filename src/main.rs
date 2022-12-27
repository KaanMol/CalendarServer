mod calendar;
mod entity;
mod errors;
mod routes;

use actix_web::{web::Data, App, HttpServer};
use ns_scraper::{route::Coordinate, route_builder::RouteFinderBuilder};

use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use std::sync::*;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
    pub client: mongodb::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create database connection
    let client_options =
        mongodb::options::ClientOptions::parse("mongodb://root:example@localhost:27017")
            .await
            .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("calendarserver");

    // Initialise the app state for Actix
    let state = AppState { client, db };

    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::user::create
        ),
        components(
            // schemas(todo::Todo, todo::TodoUpdateRequest, todo::ErrorResponse)
        ),
        tags(
            (name = "calendar", description = "The new calendar API"),
        ),
        // modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    // Create the Actix app
    let app = move || {
        App::new()
            .wrap(actix_cors::Cors::default().allow_any_origin())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .service(routes::user::create)
            .service(routes::user::read)
            .service(routes::source::create)
            .service(routes::filter::create)
            .service(routes::modifiers::create)
            .service(routes::calendar::create)
            .service(routes::calendar::read)
            .service(routes::calendar_event::create)
            .service(routes::calendar_event::read_all)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
        // .service(routes::calendar::create)
        // .service(routes::calendar::read_for_user)
    };

    // Start the Actix server
    HttpServer::new(app).bind(("0.0.0.0", 3000))?.run().await
}
