use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;
use std::fmt::Formatter;
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use actix_web::body::BoxBody;

#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum ServiceErrorStatus {
    #[error("Not Found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Unknown")]
    Unknown,
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized
}

#[derive(Serialize, Deserialize, Debug, Error)]
pub struct ServiceError {
    pub message: String,
    pub status: ServiceErrorStatus,
}

impl ServiceError {
    pub fn new(message: String, status: ServiceErrorStatus) -> Self {
        ServiceError {
            message, status
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self.status {
            ServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
            ServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            ServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            ServiceErrorStatus::Conflict => StatusCode::CONFLICT,
            ServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())

    }
}

#[macro_export]
macro_rules! error_check {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| NanoServiceError::new(
            x.to_string(),
            $err_status
        ))
    };

    ($e:expr, $err_status:expr, $message_context:expr) => {
      $e.map_err(|x| NanoServiceError::new(
          format!("{}: {}", $message_context, x.to_string()),
          $err_status
      ))
    };
}