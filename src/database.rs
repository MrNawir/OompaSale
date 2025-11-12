use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/oompasale".to_string());

    Database::connect(&database_url).await
}
