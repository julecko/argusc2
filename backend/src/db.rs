use sqlx::{Pool, MySql};

pub type Db = MySql;

pub async fn connect() -> Pool<Db> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::<Db>::connect(&url).await.expect("Failed to connect to database")
}
