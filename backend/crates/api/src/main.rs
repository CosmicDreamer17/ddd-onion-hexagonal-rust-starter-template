use application::{CreateUserCommand, CreateUserUseCase};
use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use domain::{DomainError, User};
use infra::{init_db, SqliteUserRepository};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type AppState = Arc<CreateUserUseCase<SqliteUserRepository>>;

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
    let server_port = std::env::var("PORT").unwrap_or_else(|_| "3001".into());
    let cors_origin =
        std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into());

    // 3. Initialize Domain Layers
    let pool = init_db(&database_url)
        .await
        .expect("Failed to initialize database");
    let repository = SqliteUserRepository { pool };
    let use_case: AppState = Arc::new(CreateUserUseCase { repository });

    // 4. Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(
            cors_origin
                .parse::<HeaderValue>()
                .expect("Invalid CORS_ORIGIN"),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // 5. Build Router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/register", post(register_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(use_case);

    // 6. Start Server with Graceful Shutdown
    let addr = SocketAddr::from(([127, 0, 0, 1], server_port.parse().unwrap_or(3001)));
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
    State(use_case): State<AppState>,
    Json(payload): Json<CreateUserCommand>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Received registration request for: {}", payload.email);

    let user = use_case.execute(payload).await.map_err(|e| {
        tracing::error!("Registration use case failed: {:?}", e);
        match e {
            DomainError::RepositoryError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            DomainError::ValidationError(msg) | DomainError::InvalidEmail(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            ),
        }
    })?;

    tracing::info!("User registered successfully: {}", user.id);
    Ok(Json(user))
}
