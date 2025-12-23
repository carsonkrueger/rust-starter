use axum::http::HeaderMap;

pub fn is_request(headers: &HeaderMap) -> bool {
    headers
        .get("Datastar-Request")
        .is_some_and(|v| v.to_str().is_ok_and(|v| v == "true"))
}
