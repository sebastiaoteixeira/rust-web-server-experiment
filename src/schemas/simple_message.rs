use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ResponseMessage {
    pub message: String,
}
