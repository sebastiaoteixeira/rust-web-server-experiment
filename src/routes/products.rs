use crate::models::product::Model as ProductModel;
use crate::routes::state::AppState;
use crate::schemas::product::CreateProduct;
use crate::schemas::simple_message::ResponseMessage;
use crate::services::products as product_service;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

#[axum::debug_handler]
pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    let db = &state.db;
    match product_service::create_product(db, payload.clone()).await {
        Ok(product) => {
            let response_model = ProductModel {
                id: product.last_insert_id,
                name: payload.name,
                price: payload.price,
            };
            (StatusCode::CREATED, Json(response_model)).into_response()
        }
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to create product".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_products(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;
    match product_service::get_products(db).await {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to get products".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_product_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = &state.db;
    match product_service::get_product_by_id(db, id).await {
        Ok(Some(product)) => (StatusCode::OK, Json(product)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ResponseMessage {
                message: "Product not found".to_string(),
            }),
        )
            .into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to get product".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_product(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    let db = &state.db;
    match product_service::update_product(db, id, payload).await {
        Ok(updated_product) => (StatusCode::OK, Json(updated_product)).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to update product".to_string(),
            }),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn delete_product(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = &state.db;
    match product_service::delete_product(db, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ResponseMessage {
                message: "Product deleted successfully".to_string(),
            }),
        )
            .into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseMessage {
                message: "Failed to delete product".to_string(),
            }),
        )
            .into_response(),
    }
}

pub fn create_products_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_product).get(get_products))
        .route(
            "/{id}",
            get(get_product_by_id)
                .put(update_product)
                .delete(delete_product),
        )
}
