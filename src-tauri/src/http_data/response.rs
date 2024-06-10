use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub status: u16,
    pub body: String,
}
