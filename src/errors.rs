use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;



#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    //#[display(fmt = "BadId")]
    //BadId,

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    //#[display(fmt = "ProcessError: {}", _0)]
    //ProcessError(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),
}


impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            //AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),

            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),

            //AuthError::ProcessError(ref message) => HttpResponse::InternalServerError().json(message),

            AuthError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),

            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        // We only care about UniqueViolations
        match error {
            DBError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();

                match kind {
                    DatabaseErrorKind::UniqueViolation => AuthError::DuplicateValue(message),
                    _ => AuthError::GenericError(message)
                }
            }
            _ => AuthError::GenericError(String::from("Some database error occured")),
        }
    }
}
