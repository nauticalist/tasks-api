use crate::connection::postgres::DB_POOL;

pub async fn run_migrations() {
    println!("Migrating tasks database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    let result = migrations.run(&*DB_POOL).await;
    println!(
        "tasks database migration completed: {:?}", result
    )
}