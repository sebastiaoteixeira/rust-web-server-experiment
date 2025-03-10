use crate::db::connect;
use crate::routes::state::AppState;
use axum::Router;
pub mod products;
pub mod state;
pub mod users;

pub async fn create_routes() -> Router {
    let state = AppState {
        db: connect().await,
    };

    // Combinando os routers de usuários e produtos
    Router::new()
        .nest("/users", users::create_users_router()) // Encaminha rotas de /users para o router de usuários
        .nest("/products", products::create_products_router()) // Encaminha rotas de /products para o router de produtos
        .with_state(state)
}
