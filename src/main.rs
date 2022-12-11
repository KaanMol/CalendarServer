mod calendar;
mod database;
mod errors;
mod routes;

use actix_web::{web::Data, App, HttpServer};
use ns_scraper::{route::Coordinate, route_builder::RouteFinderBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub database: sea_orm::DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let route = RouteFinderBuilder::new()
    //     .from(Coordinate::new(53.2217513, 6.530674))
    //     .to(Coordinate::new(51.5051517, 3.58268))
    //     .depart_at(chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap())
    //     .build()
    //     .expect("Failed to build route finder")
    //     .find();

    // println!("Route: {:#?}", route);
    // println!("Expected traveltime: {}m", route.travel_time.num_minutes());

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create a connection pool to the database
    let database = database::Database::connect()
        .await
        .expect("could not connect to database"); // TODO: handle this error

    // Initialise the app state for Actix
    let state = AppState { database };

    // Create the Actix app
    let app = move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    utoipa_swagger_ui::Url::new("user", "/api-doc/user.json"),
                    routes::user::ApiDoc::openapi(),
                ),
                (
                    utoipa_swagger_ui::Url::new("calendar", "/api-doc/calendar.json"),
                    routes::calendar::ApiDoc::openapi(),
                ),
            ]))
            .service(actix_web::web::scope("/user").configure(routes::user::scoped_router))
            .service(actix_web::web::scope("/calendar").configure(routes::calendar::scoped_router))
    };

    // Start the Actix server
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
