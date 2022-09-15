use crate::MUSICBEE_REST_URL;

pub(crate) fn format_url(endpoint: impl ToString) -> String {
    format!("{}/{}", MUSICBEE_REST_URL, endpoint.to_string())
}

pub(crate) fn format_url_path(endpoint: impl ToString, path: impl ToString) -> String {
    format!(
        "{}/{}?{}",
        MUSICBEE_REST_URL,
        endpoint.to_string(),
        path.to_string()
    )
}

pub(crate) fn parse_duration(ms: u32) -> String {
    let d = ms / 1000;
    let h = d / 60;
    match (d / 60) / 60 {
        0 => format!("{:02}:{:02}", (h % 60), (d % 60)),
        _ => format!("{:02}:{:02}:{:02}", (h / 60), (h % 60), (d % 60)),
    }
}
