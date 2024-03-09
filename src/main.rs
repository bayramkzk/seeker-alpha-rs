use scraper::Html;
use sqlx::postgres::PgPoolOptions;

mod database;
mod headers;
mod scrape;
mod structs;
mod user_agents;

async fn fetch_html(client: &reqwest::Client, url: &str) -> anyhow::Result<Html> {
    let builder = client.get(url).headers(headers::random_headers(url));
    let res = builder.send().await?.text().await?;
    let html = Html::parse_document(res.as_str());
    Ok(html)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().expect("No .env file found");
    let db_url = dotenvy::var("DATABASE_URL").expect("No DATABASE_URL found");
    let proxy_uri = dotenvy::var("PROXY_URI").expect("No PROXY_URI found");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await?;

    let proxy = reqwest::Proxy::http(proxy_uri)?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;

    let url = "https://seekingalpha.com/article/4614677-elite-pharmaceuticals-inc-eltp-q4-2023-earnings-call-transcript";
    let html = fetch_html(&client, url).await?;
    println!("Errors: {:?}", html.errors);
    println!("{}", html.html());

    Ok(())
}
