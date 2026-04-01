use std::future::Future;

use domain::{DomainError, Email, User, UserId, Username};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub trait UserRepository: Send + Sync {
    fn create(&self, user: User) -> impl Future<Output = Result<(), DomainError>> + Send;
    fn find_by_id(
        &self,
        id: &UserId,
    ) -> impl Future<Output = Result<Option<User>, DomainError>> + Send;
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
        let email = Email::new(cmd.email)?;
        let username = Username::new(cmd.username)?;

        let user = User {
            id: UserId::new(),
            email,
            username,
        };
        self.repository.create(user.clone()).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

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

        assert_eq!(result.email.as_str(), "test@example.com");
        assert_eq!(result.username.as_str(), "testuser");
    }

    #[tokio::test]
    async fn test_create_user_rejects_invalid_email() {
        let repo = MockUserRepository {
            users: Mutex::new(vec![]),
        };
        let use_case = CreateUserUseCase { repository: repo };

        let cmd = CreateUserCommand {
            email: "not-an-email".into(),
            username: "testuser".into(),
        };

        let result = use_case.execute(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_rejects_empty_username() {
        let repo = MockUserRepository {
            users: Mutex::new(vec![]),
        };
        let use_case = CreateUserUseCase { repository: repo };

        let cmd = CreateUserCommand {
            email: "test@example.com".into(),
            username: "".into(),
        };

        let result = use_case.execute(cmd).await;
        assert!(result.is_err());
    }
}
