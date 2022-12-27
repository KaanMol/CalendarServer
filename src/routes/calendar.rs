use crate::{AppState, entity, routes::{DataResponse, ErrorResponse, Response}};
use actix_web::{
    error,
    web::{Data, Json, Query, Path},
    Error, HttpResponse,
};
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
pub struct CreateCalendarBody {
    name: String
}

#[actix_web::post("/calendars")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateCalendarBody>,
) -> Result<HttpResponse, Error> {
    let result = state
        .db
        .collection::<entity::calendar::Calendar>("calendars")
        .insert_one(
            entity::calendar::Calendar {
                name: body.name.clone()
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create calendar", e)))?;

    Ok(HttpResponse::Created().json(DataResponse::new("Created calendar", result.inserted_id)))
}

#[actix_web::get("/calendars/{calendar_id}")]
pub async fn read(state: Data<AppState>, calendar_id: Path<String>) -> Result<HttpResponse, Error> {
    let id = crate::routes::parse_id(calendar_id.to_string())?;

    let calendar = state
        .db
        .collection::<entity::calendar::Calendar>("calendars")
        .find_one(
            mongodb::bson::doc! {
                "_id": id
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not query calendars", e)))?
        .ok_or_else(|| {
            error::ErrorNotFound(Response::new(format!(
                "Could not find calendar with id {}",
                calendar_id
            )))
        })?;

    Ok(HttpResponse::Ok().json(DataResponse::new("Found calendar", calendar)))
}
// #[actix_web::post("/calendars")]
// pub async fn create(
//     state: Data<AppState>,
//     body: Json<CreateCalendarBody>,
// ) -> Result<HttpResponse, Error> {
//     // FIXME: The same algorithm is used in multiple places to find a user by id.
//     //        This should be refactored into a function.
//     let user = User::Entity::find_by_id(body.user_id.clone())
//         .one(&state.database)
//         .await
//         .map_err(|_| error::ErrorBadRequest("Could not query users"))?
//         .ok_or_else(|| {
//             error::ErrorNotFound(format!("Could not find user with id {}", body.user_id))
//         })?;

//     let calendar = Calendar::ActiveModel {
//         id: ActiveValue::Set(Uuid::new_v4().to_string()),
//         user: ActiveValue::Set(user.id.clone()),
//         name: ActiveValue::Set(body.name.clone()),
//     }
//     .insert(&state.database)
//     .await
//     .map_err(|_| error::ErrorBadRequest("Could not create calendar"))?;

//     Ok(HttpResponse::Created().json(calendar))
// }

// #[derive(serde::Deserialize, Clone)]
// pub struct FindCalendarQuery {
//     user: String,
// }

// #[actix_web::get("/calendars")]
// pub async fn read_for_user(
//     query: Query<FindCalendarQuery>,
//     state: Data<AppState>,
// ) -> Result<HttpResponse, Error> {
//     // FIXME: The same algorithm is used in multiple places to find a user by id.
//     //        This should be refactored into a function.
//     let user: User::Model = User::Entity::find_by_id(query.user.clone())
//         .one(&state.database)
//         .await
//         .map_err(|_| error::ErrorBadRequest("Could not query users"))?
//         .ok_or_else(|| {
//             error::ErrorNotFound(format!("Could not find user with id {}", query.user))
//         })?;

//     let calendars = user
//         .find_related(Calendar::Entity)
//         .all(&state.database)
//         .await
//         .map_err(|_| error::ErrorBadRequest("Could not query calendars"))?;

//     Ok(HttpResponse::Ok().json(calendars))
// }


// // TODO: Update calendar

// // TODO: Delete calendar
