use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
