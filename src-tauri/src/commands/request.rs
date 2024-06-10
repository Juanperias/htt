use crate::{
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

    let response = match method {
        "GET" => request_get(url, parsed_headers),
        "POST" => request_post(url, body, parsed_headers),
        "PUT" => request_put(url, body, parsed_headers),
        "DELETE" => request_delete(url, body, parsed_headers),
        "PATCH" => request_patch(url, body, parsed_headers),
        _ => panic!("Method not found"),
    };

    response
}
