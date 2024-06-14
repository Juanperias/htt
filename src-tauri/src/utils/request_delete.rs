use crate::{fn_params::request_params::RequestParams, http_data::response::Response};
use ureq::OrAnyStatus;

use super::parse_res::parse_res;

pub fn request_delete(http_params: RequestParams) -> Response {
    let res = ureq::delete(http_params.url.as_str())
        .set("Content-Type", "application/json")
        .set_headers(http_params.headers)
        .send_string(http_params.body.as_str())
        .or_any_status()
        .expect("ERROR IN THE REQUEST");

    parse_res(res)
}
