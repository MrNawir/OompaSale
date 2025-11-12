use crate::database::establish_connection;
use sea_orm::DatabaseConnection;

pub struct ApiGateway {
    db: DatabaseConnection,
}

impl ApiGateway {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = establish_connection().await?;
        Ok(Self { db })
    }

    pub fn database(&self) -> &DatabaseConnection {
        &self.db
    }
}
