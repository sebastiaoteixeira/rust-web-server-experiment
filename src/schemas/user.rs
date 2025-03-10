use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
