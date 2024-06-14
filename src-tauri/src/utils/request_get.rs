use ureq::OrAnyStatus;

use crate::{fn_params::request_params::RequestParams, http_data::response::Response};

use super::parse_res::parse_res;

pub fn request_get(http_params: RequestParams) -> Response {
    let res = ureq::get(http_params.url.as_str())
        .set_headers(http_params.headers)
        .call()
        .or_any_status()
        .expect("ERROR IN THE REQUEST");

    parse_res(res)
}
