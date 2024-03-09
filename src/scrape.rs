use scraper::Html;

use crate::structs::ParsedCall;

fn parse_call_page(url: &str, html: &Html) -> ParsedCall {
    ParsedCall {
        url: url.to_string(),
        title: todo!(),
        tickers: todo!(),
        date: todo!(),
        participants: todo!(),
        transcripts: todo!(),
    }
}
