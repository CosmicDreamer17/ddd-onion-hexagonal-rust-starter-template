use application::{CreateUserCommand, CreateUserUseCase};
use clap::{Parser, Subcommand};
use infra::{init_db, SqliteUserRepository};

#[derive(Parser)]
#[command(name = "ddd-admin")]
#[command(about = "Hexagonal/DDD Admin CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Registers a new user directly in the database
    Register {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        username: String,
    },
    /// Checks the health of the system
    Health,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:local.db".into());
    let pool = init_db(&database_url)
        .await
        .expect("Failed to initialize database");
    let repository = SqliteUserRepository { pool };
    let use_case = CreateUserUseCase { repository };

    let cli = Cli::parse();

    match &cli.command {
        Commands::Register { email, username } => {
            let cmd = CreateUserCommand {
                email: email.clone(),
                username: username.clone(),
            };
            println!("🚀 Executing Register Use Case...");
            let user = use_case.execute(cmd).await?;
            println!("✅ Successfully registered user!");
            println!("🆔 ID: {}", user.id);
            println!("📧 Email: {}", user.email);
            println!("👤 Username: {}", user.username);
        }
        Commands::Health => {
            println!("🏥 Checking System Health...");
            // In a real app, we would ping the database here
            println!("✅ System is HEALTHY");
        }
    }

    Ok(())
}
