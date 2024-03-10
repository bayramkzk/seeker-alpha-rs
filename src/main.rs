use std::sync::Arc;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::{Mutex, Semaphore};

mod database;
mod headers;
mod scrape;
mod structs;
mod user_agents;

struct UrlRecord {
    url: String,
}

async fn fetch_random(pool: &Pool<Postgres>, client: &reqwest::Client) -> Result<String> {
    let urls = sqlx::query_as!(
        UrlRecord,
        "SELECT url FROM call_pages WHERE html IS NULL ORDER BY random() LIMIT 1"
    )
    .fetch_all(pool)
    .await?;
    let url: String = match urls.len() {
        0 => panic!("No more URLs to fetch"),
        1 => urls[0].url.clone(),
        _ => panic!(),
    };
    let headers = headers::random_headers(url.as_str());
    let builder = client.get(url.as_str()).headers(headers);
    let content = builder.send().await?.text().await?;
    if !content.contains(r#"data-test-id="post-title""#) {
        return Err(anyhow::Error::msg("No title element found in page"));
    }
    sqlx::query("UPDATE call_pages SET html = $1 WHERE url = $2")
        .bind(content)
        .bind(url.as_str())
        .execute(pool)
        .await?;
    Ok(url)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().expect("No .env file found");
    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL found");
    let proxy_uri = std::env::var("PROXY_URI").expect("No PROXY_URI found");
    let proxy_user = std::env::var("PROXY_USER").expect("No PROXY_USER found");
    let proxy_pass = std::env::var("PROXY_PASS").expect("No PROXY_PASS found");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await?;
    let pool = Arc::new(Mutex::from(pool));

    let proxy = reqwest::Proxy::all(proxy_uri)?.basic_auth(&proxy_user, &proxy_pass);
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let client = Arc::new(Mutex::from(client));
    let semaphore = Arc::new(Semaphore::new(10));

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let pool = pool.clone();
        let client = client.clone();

        tokio::spawn(async move {
            let client = client.lock().await;
            let pool = pool.lock().await;

            let res = fetch_random(&pool, &client).await;
            match res {
                Ok(url) => tracing::info!("Successfully fetched and saved url {url}"),
                Err(err) => tracing::warn!("Couldn't fetch random {}", err),
            }
            drop(permit);
        });
    }
}
