use crate::{calendar, AppState};
use actix_web::{
    error, get, post,
    web::{self, Data, Json, Query},
    Error, HttpResponse,
};
use entity::calendar as Calendar;
use entity::user as User;
use sea_orm::ActiveModelTrait;
use sea_orm::ModelTrait;
use sea_orm::{ActiveValue, EntityTrait};
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
        paths(
            create,
            read_for_user
        ),
        components(
            schemas(CreateCalendarBody, FindCalendarQuery)
        ),
        tags(
            (name = "calendar", description = "Calendar API")
        )
    )]
pub struct ApiDoc;

#[derive(serde::Deserialize, Clone, utoipa::ToSchema)]
struct CreateCalendarBody {
    name: String,
    user_id: String,
}

pub fn scoped_router(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(read_for_user);
}

#[utoipa::path(
    request_body = CreateCalendarBody,
    responses(
        (status = 200, description = "Create new calendar", body = Calendar::Model)
    )
)]
#[post("/")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateCalendarBody>,
) -> Result<HttpResponse, Error> {
    // FIXME: The same algorithm is used in multiple places to find a user by id.
    //        This should be refactored into a function.
    let user = User::Entity::find_by_id(body.user_id.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| {
            error::ErrorNotFound(format!("Could not find user with id {}", body.user_id))
        })?;

    let calendar = Calendar::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        user: ActiveValue::Set(user.id.clone()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .map_err(|_| error::ErrorBadRequest("Could not create calendar"))?;

    Ok(HttpResponse::Created().json(calendar))
}

#[derive(serde::Deserialize, Clone, utoipa::ToSchema)]
pub struct FindCalendarQuery {
    user: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all calendars", body = [entity::calendar::Model])
    )
)]
#[get("/")]
pub async fn read_for_user(
    query: Query<FindCalendarQuery>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    // FIXME: The same algorithm is used in multiple places to find a user by id.
    //        This should be refactored into a function.
    let user: User::Model = User::Entity::find_by_id(query.user.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| {
            error::ErrorNotFound(format!("Could not find user with id {}", query.user))
        })?;

    let calendars = user
        .find_related(Calendar::Entity)
        .all(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query calendars"))?;

    Ok(HttpResponse::Ok().json(calendars))
}

// TODO: Update calendar

// TODO: Delete calendar
