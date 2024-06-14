use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub content_type: String,
}
