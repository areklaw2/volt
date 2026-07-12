use crate::domain::errors::DomainError;
use crate::domain::ids::UserId;
use crate::domain::repository::UserRepository;
use crate::domain::user::{DisplayName, User, Username};

pub struct CreateUserCommand {
    pub id: Option<UserId>,
    pub username: String,
    pub display_name: String,
}

pub struct CreateUserHandler<U: UserRepository> {
    users: U,
}

impl<U: UserRepository> CreateUserHandler<U> {
    pub fn new(users: U) -> Self {
        Self { users }
    }

    pub async fn handle(&self, command: CreateUserCommand) -> Result<User, DomainError> {
        if let Some(id) = &command.id
            && let Some(existing) = self.users.find_by_id(id).await.map_err(|e| DomainError::Internal(e.to_string()))?
        {
            return Ok(existing);
        }

        let id = command.id.unwrap_or_default();
        let username = Username::new(command.username)?;
        let display_name = DisplayName::new(command.display_name)?;
        let user = User::new(id, username, display_name);

        self.users.save(&user).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::repository::RepoError;

    #[derive(Default)]
    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepoError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.id() == id)
                .map(|u| User::from_persistence(u.id().clone(), u.username().clone(), u.display_name().clone(), *u.created_at())))
        }

        async fn find_all(&self) -> Result<Vec<User>, RepoError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .map(|u| User::from_persistence(u.id().clone(), u.username().clone(), u.display_name().clone(), *u.created_at()))
                .collect())
        }

        async fn save(&self, user: &User) -> Result<(), RepoError> {
            self.users.lock().unwrap().push(User::from_persistence(
                user.id().clone(),
                user.username().clone(),
                user.display_name().clone(),
                *user.created_at(),
            ));
            Ok(())
        }
    }

    #[tokio::test]
    async fn handle_creates_new_user_with_generated_id() {
        let handler = CreateUserHandler::new(MockUserRepository::default());

        let user = handler
            .handle(CreateUserCommand {
                id: None,
                username: "alice".into(),
                display_name: "Alice".into(),
            })
            .await
            .unwrap();

        assert_eq!(user.username().as_str(), "alice");
    }

    #[tokio::test]
    async fn handle_returns_existing_user_when_id_already_taken() {
        let handler = CreateUserHandler::new(MockUserRepository::default());
        let id = UserId::new();

        let first = handler
            .handle(CreateUserCommand {
                id: Some(id.clone()),
                username: "alice".into(),
                display_name: "Alice".into(),
            })
            .await
            .unwrap();

        let second = handler
            .handle(CreateUserCommand {
                id: Some(id.clone()),
                username: "someone-else".into(),
                display_name: "Someone Else".into(),
            })
            .await
            .unwrap();

        assert_eq!(second.id(), first.id());
        assert_eq!(second.username().as_str(), "alice");
    }

    #[tokio::test]
    async fn handle_rejects_empty_username() {
        let handler = CreateUserHandler::new(MockUserRepository::default());

        let result = handler
            .handle(CreateUserCommand {
                id: None,
                username: "  ".into(),
                display_name: "Alice".into(),
            })
            .await;

        assert_eq!(result.err(), Some(DomainError::EmptyUsername));
    }
}
