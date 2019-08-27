use actix_web::{error::{ResponseError, ParseError, PayloadError, JsonPayloadError, ContentTypeError}, http, HttpResponse};
use derive_more::Display;
use serde_derive::Serialize;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;

use crate::failure;

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),

    #[fail(display = "Not Found")]
    NotFound(String),

    #[fail(display = "Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponseModel {
    code: i32,
    message: String,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                let err = ErrorResponseModel {
                    code: 500,
                    message: "Internal Server Error, Please try later".to_string(),
                };
                HttpResponse::InternalServerError().json(err)
            }
            ServiceError::BadRequest(ref message) => {
                let err = ErrorResponseModel {
                    code: 400,
                    message: message.to_string(),
                };
                HttpResponse::BadRequest().json(err)
            }
            ServiceError::Unauthorized => {
                let err = ErrorResponseModel {
                    code: 401,
                    message: "Unauthorized".to_string(),
                };
                HttpResponse::Unauthorized().json(err)
            }
            ServiceError::NotFound(ref message) => {
                let err = ErrorResponseModel {
                    code: 404,
                    message: message.to_string(),
                };
                HttpResponse::NotFound().json(err)
            }
        }
    }
}


impl From<ParseError> for ServiceError {
    fn from(err: ParseError) -> ServiceError {
        ServiceError::BadRequest(format!("Bad Request: {}", err.to_string()).to_string())
    }
}

impl From<PayloadError> for ServiceError {
    fn from(err: PayloadError) -> ServiceError {
        ServiceError::BadRequest(err.to_string())
    }
}

impl From<JsonPayloadError> for ServiceError {
    fn from(err: JsonPayloadError) -> ServiceError {
        ServiceError::BadRequest(err.to_string())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}