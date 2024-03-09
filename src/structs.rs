pub struct ParsedCall {
    pub url: String,
    pub title: String,
    pub tickers: Option<String>,
    pub date: String,
    pub participants: Vec<(String, String)>, // (org, name)
    pub transcripts: Vec<(String, String)>,  // (speaker, content)
}
