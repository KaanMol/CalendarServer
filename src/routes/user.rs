use crate::{
    entity,
    routes::{DataResponse, ErrorResponse, Response},
    AppState,
};
use actix_web::{
    error,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use utoipa::ToSchema;

#[derive(serde::Deserialize, Clone, ToSchema)]
pub struct CreateUserBody {
    name: String,
}

#[utoipa::path(
    post,
    path = "/users",
    responses(
        (status = 201, description = "User created successfully", body = CreateUserBody),
        // (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
#[actix_web::post("/users")]
pub async fn create(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<HttpResponse, Error> {
    let result = state
        .db
        .collection::<entity::user::User>("users")
        .insert_one(
            entity::user::User {
                name: body.name.clone(),
                sources: vec![],
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not create user", e)))?;

    Ok(HttpResponse::Created().json(DataResponse::new("Created user", result.inserted_id)))
}

#[actix_web::get("/users/{user_id}")]
pub async fn read(state: Data<AppState>, user_id: Path<String>) -> Result<HttpResponse, Error> {
    let id = crate::routes::parse_id(user_id.to_string())?;

    let user = state
        .db
        .collection::<entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "_id": id
            },
            None,
        )
        .await
        .map_err(|e| error::ErrorBadRequest(ErrorResponse::new("Could not query users", e)))?
        .ok_or_else(|| {
            error::ErrorNotFound(Response::new(format!(
                "Could not find user with id {}",
                user_id
            )))
        })?;

    Ok(HttpResponse::Ok().json(DataResponse::new("Found user", user)))
}

// // TODO: Update user

// // TODO: Delete user
