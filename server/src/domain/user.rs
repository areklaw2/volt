use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{errors::DomainError, ids::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::EmptyUsername);
        }
        Ok(Self(value))
    }

    pub(crate) fn from_persistence(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::EmptyDisplayName);
        }
        Ok(Self(value))
    }

    pub(crate) fn from_persistence(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Getters, PartialEq)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub")]
    username: Username,
    #[getset(get = "pub")]
    display_name: DisplayName,
    #[getset(get = "pub")]
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, username: Username, display_name: DisplayName) -> Self {
        Self {
            id,
            username,
            display_name,
            created_at: Utc::now(),
        }
    }

    pub(crate) fn from_persistence(id: UserId, username: Username, display_name: DisplayName, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            username,
            display_name,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_rejects_empty_value() {
        let result = Username::new("  ".into());

        assert_eq!(result.err(), Some(DomainError::EmptyUsername));
    }

    #[test]
    fn display_name_rejects_empty_value() {
        let result = DisplayName::new("  ".into());

        assert_eq!(result.err(), Some(DomainError::EmptyDisplayName));
    }

    #[test]
    fn new_builds_user_with_correct_fields() {
        let id = UserId::new();
        let username = Username::new("alice".into()).unwrap();
        let display_name = DisplayName::new("Alice Smith".into()).unwrap();

        let user = User::new(id.clone(), username.clone(), display_name.clone());

        assert_eq!(user.id(), &id);
        assert_eq!(user.username(), &username);
        assert_eq!(user.display_name(), &display_name);
    }

    #[test]
    fn from_persistence_reconstructs_user_fields() {
        let id = UserId::new();
        let username = Username::from_persistence("alice".into());
        let display_name = DisplayName::from_persistence("Alice Smith".into());
        let created_at = Utc::now();

        let user = User::from_persistence(id.clone(), username.clone(), display_name.clone(), created_at);

        assert_eq!(user.id(), &id);
        assert_eq!(user.username(), &username);
        assert_eq!(user.display_name(), &display_name);
        assert_eq!(user.created_at(), &created_at);
    }
}
