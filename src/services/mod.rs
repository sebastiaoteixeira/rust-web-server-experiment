pub mod products;
pub mod users;

pub enum ServiceError {
    NotFound,
    InternalServerError,
}
