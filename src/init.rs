use std::str::FromStr;
use std::time::Duration;
use sqlx::{ConnectOptions, PgPool};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {

    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting global default subscriber failed");

}

pub async fn database_connection() -> PgPool {
    tracing::debug!("Setting up database connection");
    let db_url = dotenvy::var("DATABASE_URL").expect("failed to get database URL from env file");

    let options = PgConnectOptions::from_str(&db_url)
     .expect("failed to parse database URL")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("failed to connect to database");

    tracing::debug!("Successfully connected to database");

    sqlx::migrate!()
        .run( &pg_pool)
        .await
        .expect("failed to migrate");
    tracing::debug!("Successfully migrated database");

    pg_pool
}