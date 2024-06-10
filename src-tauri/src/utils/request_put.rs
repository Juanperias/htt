use crate::http_data::response::Response;
use ureq::OrAnyStatus;

pub fn request_put(url: &str, body: &str, headers: Vec<(String, String)>) -> Response {
    let res = ureq::put(url)
        .set("Content-Type", "application/json")
        .set_headers(headers)
        .send_string(body)
        .or_any_status()
        .expect("ERROR IN THE REQUEST");

    Response {
        status: res.status(),
        body: res.into_string().expect("ERROR IN THE RESPONSE"),
    }
}
