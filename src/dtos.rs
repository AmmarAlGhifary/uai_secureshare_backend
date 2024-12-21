use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::{ReceiverFileDetails, SendFileDetails, User};

// DTOs (Data Transfer Objects) for user registration
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    // User's name, required
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    // User's email, required and must follow valid email format
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    // User's password, required and must be at least 6 characters long
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,

    // Confirmation password, must match the password
    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message="passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

// DTO for user login
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    // Email is required and must follow a valid format
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,

    // Password is required and must be at least 6 characters
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

// DTO for request query parameters, with optional pagination
#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    // Page number, must be greater than 0
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    // Limit for items per page, must be between 1 and 50
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

// Filter user information for public-facing responses
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub public_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    // Creates a `FilterUserDto` from a `User` model
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            public_key: user.public_key.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

// Wrapper for filtered user data
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

// Response DTO for user-related actions
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

// DTO for sending file details
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
    pub file_id: String,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl UserSendFileDto {
    // Map `SendFileDetails` to `UserSendFileDto`
    pub fn filter_send_user_file(file_data: &SendFileDetails) -> Self {
        UserSendFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    // Process a list of `SendFileDetails` to `UserSendFileDto`
    pub fn filter_send_user_files(user: &[SendFileDetails]) -> Vec<UserSendFileDto> {
        user.iter().map(UserSendFileDto::filter_send_user_file).collect()
    }
}

// Response DTO for file sending list
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileListResponseDto {
    pub status: String,
    pub files: Vec<UserSendFileDto>,
    pub results: i64,
}

// DTO for received file details
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileDto {
    pub file_id: String,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl UserReceiveFileDto {
    // Map `ReceiverFileDetails` to `UserReceiveFileDto`
    pub fn filter_receive_user_file(file_data: &ReceiverFileDetails) -> Self {
        UserReceiveFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            sender_email: file_data.sender_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    // Process a list of `ReceiverFileDetails` to `UserReceiveFileDto`
    pub fn filter_receive_user_files(user: &[ReceiverFileDetails]) -> Vec<UserReceiveFileDto> {
        user.iter().map(UserReceiveFileDto::filter_receive_user_file).collect()
    }
}

// Response DTO for received file list
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileListResponseDto {
    pub status: String,
    pub files: Vec<UserReceiveFileDto>,
    pub results: i64,
}

// Response DTO for user login
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

// General response structure
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

// DTO for updating user name
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

// DTO for updating user password
#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "new password must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New password confirm is required."),
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required."),
        length(min = 6, message = "Old password must be at least 6 characters")
    )]
    pub old_password: String,
}

// DTO for searching users by email
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchQueryByEmailDTO {
    #[validate(length(min = 1, message = "Query is required"))]
    pub query: String,
}

// Filter email for public-facing responses
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterEmailDto {
    pub email: String,
}

impl FilterEmailDto {
    // Map a `User` to `FilterEmailDto`
    pub fn filter_email(user: &User) -> Self {
        FilterEmailDto {
            email: user.email.to_owned(),
        }
    }

    // Process a list of `User` to `FilterEmailDto`
    pub fn filter_emails(user: &[User]) -> Vec<FilterEmailDto> {
        user.iter().map(FilterEmailDto::filter_email).collect()
    }
}

// Response DTO for email list
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponseDto {
    pub status: String,
    pub emails: Vec<FilterEmailDto>,
}

// DTO for file uploads with validation
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileUploadDtos {
    #[validate(email(message = "Invalid email format"))]
    pub recipient_email: String,

    #[validate(
        length(min = 1, message = "Password is required."),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,

    #[validate(custom = "validate_expiration_date")]
    pub expiration_date: String,
}

// Validate expiration date for file uploads
fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
    if expiration_date.is_empty() {
        let mut error = ValidationError::new("expiration_date_required");
        error.message = Some("Expiration date is required.".into());
        return Err(error);
    }

    // Parse date and validate if it is in the future
    let parsed_date = DateTime::parse_from_rfc3339(expiration_date).map_err(|_| {
        let mut error = ValidationError::new("invalid_date_format");
        error.message = Some("Invalid date format. Expected format is YYYY-MM-DDTHH:MM:SS.ssssssZ.".into());
        error
    })?;

    let now = Utc::now();

    if parsed_date <= now {
        let mut error = ValidationError::new("expiration_date_future");
        error.message = Some("Expiration date must be in the future.".into());
        return Err(error);
    }

    Ok(())
}

// DTO for retrieving files with validation
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RetrieveFileDto {
    #[validate(length(min = 1, message = "Shared id is required"))]
    pub shared_id: String,

    #[validate(
        length(min = 1, message = "Password is required."),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}
