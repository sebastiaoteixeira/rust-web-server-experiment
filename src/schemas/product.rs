use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
}
