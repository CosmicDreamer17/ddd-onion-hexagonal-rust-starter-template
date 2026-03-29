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
            id: UserId::new(), // Backend-driven ID generation
            email: Email(cmd.email),
            username: Username(cmd.username),
        };
        self.repository.create(user.clone()).await?;
        Ok(user)
    }
}
