use crate::MUSICBEE_REST_URL;

pub fn format_url(endpoint: impl ToString) -> String {
    format!("{}/{}", MUSICBEE_REST_URL, endpoint.to_string())
}

pub fn format_url_path(endpoint: impl ToString, path: impl ToString) -> String {
    format!(
        "{}/{}?{}",
        MUSICBEE_REST_URL,
        endpoint.to_string(),
        path.to_string()
    )
}
