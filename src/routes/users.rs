use crate::models::user::Model as UserModel;
use crate::schemas::simple_message::ResponseMessage;
use crate::schemas::user::CreateUser;
use crate::services::users as user_service;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use super::state::AppState;

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let db = &state.db;

    match user_service::create_user(db, payload.clone()).await {
        Ok(insert_result) => {
            let response_model = UserModel {
                id: insert_result.id,
                name: payload.name,
                email: payload.email,
            };
            (StatusCode::CREATED, Json(response_model)).into_response()
        }
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to create user".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;
    match user_service::get_users(db).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to get users".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = &state.db;
    match user_service::get_user_by_id(db, id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ResponseMessage {
                message: "User not found".to_string(),
            }),
        )
            .into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to get user".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let db = &state.db;
    match user_service::update_user(db, id, payload).await {
        Ok(updated_user) => (StatusCode::OK, Json(updated_user)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to update user".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn delete_user(Path(id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;
    match user_service::delete_user(db, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ResponseMessage {
                message: "User deleted successfully".to_string(),
            }),
        )
            .into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to delete user".to_string(),
            }),
        )
            .into_response(),
    }
}

pub fn create_users_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route(
            "/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}
