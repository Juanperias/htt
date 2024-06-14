use crate::http_data::response::Response;
use ureq::Response as ResponseType;

pub fn parse_res(res: ResponseType) -> Response {
    Response {
        status: res.status(),
        content_type: res.content_type().to_string(),
        body: res.into_string().expect("ERROR IN THE REQUEST"),
    }
}
