use crate::AppState;
use actix_web::{
    error, get, post,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use entity::user as User;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
        paths(
            create,
            read_all,
            read
        ),
        components(
            schemas(CreateUserBody)
        ),
        tags(
            (name = "user", description = "User API")
        )
    )]
pub struct ApiDoc;

pub fn scoped_router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create).service(read_all).service(read);
}

#[derive(serde::Deserialize, Clone, utoipa::ToSchema)]
pub struct CreateUserBody {
    name: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Created user", body = [entity::calendar::Model])
    )
)]
#[post("/")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<HttpResponse, Error> {
    let user = User::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        name: ActiveValue::Set(body.name.clone()),
    }
    .insert(&state.database)
    .await
    .map_err(|_| error::ErrorBadRequest("Could not create user"))?;

    Ok(HttpResponse::Created().json(user))
}

#[utoipa::path(
    responses(
        (status = 200, description = "List of all users", body = [entity::calendar::Model])
    )
)]
#[get("/")]
pub async fn read_all(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let users = User::Entity::find()
        .all(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?;

    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    params(
        ("user_id", description = "UUID of the user")
    ),
    responses(
        (status = 200, description = "User object by UserID", body = [entity::calendar::Model])
    )
)]
#[get("/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    let user = User::Entity::find_by_id(user_id.clone())
        .one(&state.database)
        .await
        .map_err(|_| error::ErrorBadRequest("Could not query users"))?
        .ok_or_else(|| error::ErrorNotFound(format!("Could not find user with id {}", user_id)))?;

    Ok(HttpResponse::Ok().json(user))
}

// TODO: Update user

// TODO: Delete user
