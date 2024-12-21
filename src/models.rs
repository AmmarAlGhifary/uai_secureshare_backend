use chrono::Utc;
use rsa::pkcs8::der::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub id: uuid::Uuid,
    pub nama: String,
    pub password: String,
    pub public_key: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
} 

#[derive(Debug, Clone, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct File{
    pub id: uuid::Uuid, 
    pub user_id: Option<uuid::Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub encrypted_aes_key:Vec<u8>,
    pub encrypted_aes_key: Vec<u8>,
}