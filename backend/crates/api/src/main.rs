use application::{CreateUserCommand, CreateUserUseCase};
use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Extension, Json, Router,
};
use domain::{DomainError, User};
use infra::{init_db, SqliteUserRepository};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Initialize Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Load Environment Variables
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:local.db".into());
    let server_port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());

    // 3. Initialize Domain Layers
    let pool = init_db(&database_url)
        .await
        .expect("Failed to initialize database");
    let repository = SqliteUserRepository { pool };
    let use_case = Arc::new(CreateUserUseCase { repository });

    // 4. Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // 5. Build Router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/register", post(register_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(use_case));

    // 6. Start Server with Graceful Shutdown
    let addr = SocketAddr::from(([127, 0, 0, 1], server_port.parse().unwrap_or(3000)));
    tracing::info!("Server listening on {}...", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown...");
}

async fn health_handler() -> StatusCode {
    StatusCode::OK
}

async fn register_handler(
    Extension(use_case): Extension<Arc<CreateUserUseCase<SqliteUserRepository>>>,
    Json(payload): Json<CreateUserCommand>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Received registration request for: {}", payload.email);

    let user = use_case.execute(payload).await.map_err(|e| {
        tracing::error!("Registration use case failed: {:?}", e);
        match e {
            DomainError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            DomainError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            ),
        }
    })?;

    tracing::info!("User registered successfully: {:?}", user.id);
    Ok(Json(user))
}
