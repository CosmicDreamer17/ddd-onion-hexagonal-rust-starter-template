use axum::{
    routing::post,
    Json, Router, Extension,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::sync::Arc;
use domain::{User, DomainError};
use application::{CreateUserCommand, CreateUserUseCase};
use infra::{SqliteUserRepository, init_db};

#[tokio::main]
async fn main() {
    let database_url = "sqlite://local.db";
    let pool = init_db(database_url).await.expect("Failed to initialize database");
    let repository = SqliteUserRepository { pool };
    let use_case = Arc::new(CreateUserUseCase { repository });

    let app = Router::new()
        .route("/register", post(register_handler))
        .layer(Extension(use_case));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}...", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register_handler(
    Extension(use_case): Extension<Arc<CreateUserUseCase<SqliteUserRepository>>>,
    Json(payload): Json<CreateUserCommand>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = use_case.execute(payload).await.map_err(|e| match e {
        DomainError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        DomainError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error".to_string()),
    })?;

    Ok(Json(user))
}
