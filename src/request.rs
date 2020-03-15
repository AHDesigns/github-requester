extern crate reqwest;

pub mod github;
use github::PullRequestsData;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::env;

pub async fn make_request(body: Value) -> Result<PullRequestsData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    println!("sending request: {:#?}", &body);

    let request_url = "https://api.github.com/graphql";

    let response = client
        .post(request_url)
        .header(USER_AGENT, "learn-rust")
        .header(
            AUTHORIZATION,
            format!("bearer {}", env::var("GH_SKY_ACCESS_TOKEN")?),
        )
        .json(&body)
        .send()
        .await?;

    Ok(response.json().await?)
}
