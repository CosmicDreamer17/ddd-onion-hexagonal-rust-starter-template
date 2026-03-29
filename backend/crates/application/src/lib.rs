use async_trait::async_trait;
use domain::{DomainError, Email, User, UserId, Username};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub username: String,
}

pub struct CreateUserUseCase<R: UserRepository> {
    pub repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    #[instrument(skip(self, cmd), fields(email = %cmd.email))]
    pub async fn execute(&self, cmd: CreateUserCommand) -> Result<User, DomainError> {
        let user = User {
            id: UserId::new(),
            email: Email(cmd.email),
            username: Username(cmd.username),
        };
        self.repository.create(user.clone()).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // A simple MockRepository for unit testing the Use Case
    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: User) -> Result<(), DomainError> {
            self.users.lock().unwrap().push(user);
            Ok(())
        }
        async fn find_by_id(&self, _id: &UserId) -> Result<Option<User>, DomainError> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_create_user_use_case() {
        let repo = MockUserRepository {
            users: Mutex::new(vec![]),
        };
        let use_case = CreateUserUseCase { repository: repo };

        let cmd = CreateUserCommand {
            email: "test@example.com".into(),
            username: "testuser".into(),
        };

        let result = use_case.execute(cmd).await.unwrap();

        assert_eq!(result.email.0, "test@example.com");
        assert_eq!(result.username.0, "testuser");
    }
}
