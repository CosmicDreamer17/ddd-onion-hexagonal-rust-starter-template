use application::UserRepository;
use domain::{User, UserId, Email, Username, DomainError};
use async_trait::async_trait;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::str::FromStr;

pub struct SqliteUserRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create(&self, user: User) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO users (id, email, username) VALUES (?, ?, ?)"
        )
        .bind(user.id.0)
        .bind(user.email.0)
        .bind(user.username.0)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, email, username FROM users WHERE id = ?"
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
        .connect_with(options)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (id TEXT PRIMARY KEY, email TEXT NOT NULL, username TEXT NOT NULL)"
    )
    .execute(&pool)
    .await
    .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

    Ok(pool)
}
