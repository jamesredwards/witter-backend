use dotenv;
use sqlx::{query, PgPool, Pool};

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool: PgPool = Pool::new(&db_url).await?;

    let rows = query!("select 1 as one").fetch_one(&db_pool).await?;

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello, world!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
