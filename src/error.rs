use thiserror::Error;
use actix_web::http::StatusCode;
use actix_web::{ResponseError, HttpResponse};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal Server Error: error connecting to backend")]
    ConnectionError(#[from] reqwest::Error),
    #[error("Internal Server Error: backend returned an error")]
    ApiError(serde_json::Value),
    #[error("Error rendering page")]
    RenderError(#[from] handlebars::RenderError),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ConnectionError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ApiError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RenderError(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string()) // TODO: error page
    }
}
