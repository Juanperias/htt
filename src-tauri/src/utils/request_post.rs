use crate::{fn_params::request_params::RequestParams, http_data::response::Response};
use ureq::OrAnyStatus;

use super::parse_res::parse_res;

pub fn request_post(http_data: RequestParams) -> Response {
    let res = ureq::post(http_data.url.as_str())
        .set("Content-Type", "application/json")
        .set_headers(http_data.headers)
        .send_string(http_data.body.as_str())
        .or_any_status()
        .expect("ERROR IN THE REQUEST");

    parse_res(res)
}
