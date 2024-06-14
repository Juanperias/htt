use crate::{
    fn_params::request_params::RequestParams,
    http_data::{header::Header, response::Response},
    utils::{
        request_delete::request_delete, request_get::request_get, request_patch::request_patch,
        request_post::request_post, request_put::request_put,
    },
};

#[tauri::command]
pub fn make_request(url: &str, headers: Vec<Header>, body: &str, method: &str) -> Response {
    let mut parsed_headers: Vec<(String, String)> = Vec::new();

    headers.into_iter().for_each(|header| {
        parsed_headers.push((header.name, header.value));
    });

    let params = RequestParams {
        url: url.to_string(),
        body: body.to_string(),
        headers: parsed_headers,
    };

    match method {
        "GET" => request_get(params),
        "POST" => request_post(params),
        "PUT" => request_put(params),
        "DELETE" => request_delete(params),
        "PATCH" => request_patch(params),
        _ => panic!("Method not found"),
    }
}
