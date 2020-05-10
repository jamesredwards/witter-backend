use dotenv;

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
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
}
