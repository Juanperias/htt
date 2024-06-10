use ureq::OrAnyStatus;

use crate::http_data::response::Response;

pub fn request_get(url: &str, headers: Vec<(String, String)>) -> Response {
    let res = ureq::get(url)
        .set_headers(headers)
        .call()
        .or_any_status()
        .expect("ERROR IN THE REQUEST");

    Response {
        status: res.status(),
        body: res.into_string().expect("FAIL IN THE RESPONSE"),
    }
}
