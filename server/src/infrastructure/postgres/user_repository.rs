use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::ids::UserId;
use crate::domain::repository::{RepoError, UserRepository};
use crate::domain::user::{DisplayName, User, Username};

#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepoError> {
        let row = sqlx::query!(
            "SELECT id, username, display_name, created_at FROM users WHERE id = $1",
            Uuid::from(id.clone())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            User::from_persistence(
                UserId::from_persistence(r.id),
                Username::from_persistence(r.username),
                DisplayName::from_persistence(r.display_name),
                r.created_at,
            )
        }))
    }

    async fn find_all(&self) -> Result<Vec<User>, RepoError> {
        let rows = sqlx::query!("SELECT id, username, display_name, created_at FROM users ORDER BY username").fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                User::from_persistence(
                    UserId::from_persistence(r.id),
                    Username::from_persistence(r.username),
                    DisplayName::from_persistence(r.display_name),
                    r.created_at,
                )
            })
            .collect())
    }

    async fn save(&self, user: &User) -> Result<(), RepoError> {
        sqlx::query!(
            "INSERT INTO users (id, username, display_name, created_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET display_name = $3",
            Uuid::from(user.id().clone()),
            user.username().as_str(),
            user.display_name().as_str(),
            *user.created_at()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
