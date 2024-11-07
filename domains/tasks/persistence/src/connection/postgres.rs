use std::env;
use once_cell::sync::Lazy;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
    let connection_string = env::var("DB_URL").unwrap();
    let max_connection = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .trim()
        .parse::<u32>()
        .map_err(|_e| "Could not parse max connection".to_string())
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(max_connection);

    pool.connect_lazy(&connection_string).expect("Failed to create db pool")
});