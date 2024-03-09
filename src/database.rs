use sqlx::{Pool, Postgres};

use crate::structs::ParsedCall;
use anyhow::Result;

pub async fn save_fetched_html(pool: &Pool<Postgres>, url: &str, html: &str) -> Result<()> {
    sqlx::query!("UPDATE call_pages SET html = $1 WHERE url = $2", html, url)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn save_parsed_call(pool: &Pool<Postgres>, call: &ParsedCall) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "INSERT INTO call_headers (url, title, date, tickers) VALUES ($1, $2, $3, $4)",
        call.url,
        call.title,
        call.date,
        call.tickers
    )
    .execute(&mut *tx)
    .await?;

    for (org, name) in call.participants.iter() {
        sqlx::query!(
            "INSERT INTO call_participants (url, org, name) VALUES ($1, $2, $3)",
            call.url,
            org,
            name,
        )
        .execute(&mut *tx)
        .await?;
    }

    for (speaker, content) in call.transcripts.iter() {
        sqlx::query!(
            "INSERT INTO call_transcripts (url, speaker, content) VALUES ($1, $2, $3)",
            call.url,
            speaker,
            content
        )
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query!(
        "UPDATE call_pages SET parsed = true WHERE url = $1",
        call.url
    )
    .execute(&mut *tx)
    .await?;

    Ok(())
}
