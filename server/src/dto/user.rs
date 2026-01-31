use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub id: Option<String>,
    pub username: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
}
