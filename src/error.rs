use std::fmt;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

/// Structure to represent error responses sent to the client
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    /// Formats the `ErrorResponse` into a JSON string for better readability
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

/// Enum to represent various error messages in the application
#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLenght(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided
}

impl ToString for ErrorMessage {
    /// Converts `ErrorMessage` enum variant to a string
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    /// Returns a human-readable string representation for each `ErrorMessage` variant
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ExceededMaxPasswordLenght(max_length) => format!("Password must not be more than {} characters", max_length),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(),
        }
    }
}

/// Structure to encapsulate HTTP errors with a message and status code
#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    /// Constructs a new `HttpError` with a given message and status code
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError { 
            message: message.into(), 
            status
        }
    }

    /// Helper to create an internal server error
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(), 
            status: StatusCode::INTERNAL_SERVER_ERROR
         }
    }

    /// Helper to create a bad request error
    pub fn bad_reqest(message: impl Into<String>) -> Self{
        HttpError { 
            message: message.into(), 
            status: StatusCode::BAD_REQUEST, 
        }
    }

    /// Helper to create a unique constraint violation error
    pub fn unique_constrain_violation(message: impl Into<String>) -> Self{
        HttpError { 
            message: message.into(), 
            status: StatusCode::CONFLICT,
        }
    }

    /// Helper to create an unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(),
            status: StatusCode::UNAUTHORIZED, 
        }
    }

    /// Converts the `HttpError` to an HTTP response with a JSON body
    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });
        
        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    /// Formats the `HttpError` into a human-readable string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    /// Converts `HttpError` directly into an HTTP response
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
