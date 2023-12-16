use crate::HeaderMap;

#[derive(Debug, Clone)]
pub struct Response {
    pub body: Option<String>,
    pub status_code: u16,
    pub headers: HeaderMap,
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            body: None,
            status_code: 200,
            headers: HeaderMap::new(),
        }
    }

    pub fn status_code(self, status_code: u16) -> Self {
        Self { status_code, ..self }
    }

    pub fn body(self, body: impl Into<Option<String>>) -> Self {
        Self {
            body: body.into(),
            ..self
        }
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new().body(self)
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> Self {
        self
    }
}
