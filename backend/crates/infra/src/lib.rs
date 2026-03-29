use application::UserRepository;
use async_trait::async_trait;
use domain::{DomainError, Email, User, UserId, Username};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::instrument;

pub struct SqliteUserRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    #[instrument(skip(self, user), fields(user_id = %user.id.0))]
    async fn create(&self, user: User) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO users (id, email, username) VALUES (?, ?, ?)")
            .bind(user.id.0)
            .bind(user.email.0)
            .bind(user.username.0)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self, id), fields(user_id = %id.0))]
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, email, username FROM users WHERE id = ?",
        )
        .bind(id.0.clone())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(|(id, email, username)| User {
            id: UserId(id),
            email: Email(email),
            username: Username(username),
        }))
    }
}

pub async fn init_db(database_url: &str) -> Result<SqlitePool, DomainError> {
    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

    // Run migrations from the migrations/ directory in the backend root
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

    Ok(pool)
}
