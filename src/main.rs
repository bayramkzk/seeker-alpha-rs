use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("No .env file found");
    let db_url = dotenvy::var("DATABASE_URL").expect("No DATABASE_URL found");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await?;

    let row: (String,) = sqlx::query_as("SELECT url FROM call_pages")
        .fetch_one(&pool)
        .await?;

    let ip = reqwest::get("https://ip.oxylabs.io").await?.text().await?;
    let ip = ip.trim_end();

    println!("IP: {}", ip);
    println!("URL: {}", row.0);

    Ok(())
}
