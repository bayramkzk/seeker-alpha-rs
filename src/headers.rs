use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};
use url::form_urlencoded;
use uuid::Uuid;

use crate::user_agents::USER_AGENTS;

fn random_user_agent() -> String {
    USER_AGENTS
        .choose(&mut rand::thread_rng())
        .expect("No user agents found to randomly choose from")
        .to_string()
}

fn random_with_n_digits(num_digits: u32) -> u64 {
    let p = 10u64.pow(num_digits - 1);
    rand::thread_rng().gen_range(p..10 * p)
}

fn random_visited_page(pathname: &str) -> String {
    let s = format!(
        r#"LAST_VISITED_PAGE={{"pathname":"{}","pageKey":"{}"}}"#,
        pathname,
        Uuid::new_v4(),
    );
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn random_cookie(url: &str) -> String {
    let mut cookies = vec![
        format!("session_id={}", Uuid::new_v4()),
        format!("_sasource=\"\""),
        format!("machine_code={}", random_with_n_digits(13)),
        random_visited_page(url),
    ];
    cookies.shuffle(&mut thread_rng());
    cookies.join("; ")
}

pub fn random_headers(url: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, random_user_agent().parse().unwrap());
    headers.insert(COOKIE, random_cookie(url).parse().unwrap());
    headers
}
