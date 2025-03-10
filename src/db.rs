use dotenv::dotenv;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityTrait, Schema};

use crate::models::*;

async fn create_table<E>(db: &DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(
        Schema::new(builder)
            .create_table_from_entity(entity)
            .if_not_exists(),
    );

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DatabaseConnection) {
    create_table(db, UserEntity).await;
    create_table(db, ProductEntity).await;
}

static mut DB: Option<DatabaseConnection> = None;

pub async fn connect() -> DatabaseConnection {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(&db_url).await.unwrap()
}
