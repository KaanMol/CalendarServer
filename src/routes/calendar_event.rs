use crate::{AppState, entity, routes::{DataResponse, ErrorResponse, Response}};
use actix_web::{
    error,
    web::{Data, Json, Query, Path},
    Error, HttpResponse, Responder,
};
use futures::{StreamExt, TryStreamExt};
use mongodb::options::FindOptions;

#[derive(serde::Deserialize, Clone)]
pub struct CreateCalendarEventBody {
    user_id: String,
    title: String,
    description: String,
    // start: String,
    // end: String,
    all_day: bool,
    location: String,
    // repeat TODO: implement repeat
    // metadata TODO: implement metadata
    // original_event TODO: implement original_event
}

#[actix_web::post("/events")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateCalendarEventBody>,
) -> Result<HttpResponse, Error> {
    let result = state
        .db
        .collection::<entity::calendar_event::CalendarEvent>("calendar_events")
        .insert_one(
            entity::calendar_event::CalendarEvent {
                user_id: body.user_id.clone(),
                title: body.title.clone(),
                description: body.description.clone(),
                all_day: body.all_day,
                location: body.location.clone()
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create event", e)))?;

    Ok(HttpResponse::Created().json(DataResponse::new("Created event", result.inserted_id)))
}

#[actix_web::get("/events/{user_id}")]
pub async fn read_all(state: Data<AppState>, user_id: Path<String>) -> impl Responder {
    let id = crate::routes::parse_id(user_id.to_string()).unwrap();

    // let mut events = state
    //     .db
    //     .collection::<entity::calendar_event::CalendarEvent>("calendar_events")
    //     .find(
    //         mongodb::bson::doc! {
    //             "user_id": id
    //         },
    //         None,
    //     ).await.unwrap();

    //     while let Some(doc) = events.next().await {
    //         println!("{}", doc.unwrap())
    //       }
        println!("id: {}", id);
    let filter =  mongodb::bson::doc! { "user_id": id.to_string() };
    let find_options = FindOptions::builder().projection(mongodb::bson::doc! { "user_id": 1 }).build();
    let mut cursor = state
    .db
    .collection::<entity::calendar_event::CalendarEvent>("calendar_events").find(filter, find_options).await.unwrap();
    // Ok(HttpResponse::Ok().json(DataResponse::new("Found events", "hi")))
   // loop through the cursor
    while let Some(book) = cursor.try_next().await.unwrap() {
        println!("title: {}", book.title);
    }

    // Ok(HttpResponse::Ok().json(DataResponse::new("Found events", events)))

    "ok"
}