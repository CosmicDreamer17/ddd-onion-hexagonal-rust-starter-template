use application::UserRepository;
use domain::{DomainError, Email, User, UserId, Username};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::instrument;

pub struct SqliteUserRepository {
    pub pool: SqlitePool,
}

impl UserRepository for SqliteUserRepository {
    #[instrument(skip(self, user), fields(user_id = %user.id))]
    async fn create(&self, user: User) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO users (id, email, username) VALUES (?, ?, ?)")
            .bind(user.id.as_str())
            .bind(user.email.as_str())
            .bind(user.username.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self, id), fields(user_id = %id))]
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, email, username FROM users WHERE id = ?",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(row.map(|(id, email, username)| User {
            id: UserId::from_persisted(id),
            email: Email::from_persisted(email),
            username: Username::from_persisted(username),
        }))
    }
}

pub async fn init_db(database_url: &str) -> Result<SqlitePool, DomainError> {
    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

    // Run migrations from the migrations/ directory in the backend root
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

    Ok(pool)
}
