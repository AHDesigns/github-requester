extern crate reqwest;

pub mod github;
use github::{GetPrArgs, PullRequestsData};

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::env;

pub async fn make_request(
    after: &Option<String>,
) -> Result<PullRequestsData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let name = env::var("REPO_NAME").expect("missing env var REPO_NAME");
    let org = env::var("ORG_NAME").expect("missing env var ORG_NAME");

    println!("fetching info for {}/{}", &org, &name);

    let config = GetPrArgs {
        name: name,
        owner: org,
        after: after.clone(),
    };

    let body = json!({ "query": &github::query(config) });

    let request_url = "https://api.github.com/graphql";

    println!("sending request: {:#?}", &body);

    let token = env::var("ACCESS_TOKEN").expect("missing env var ACCESS_TOKEN");

    let response = client
        .post(request_url)
        .header(USER_AGENT, "learn-rust")
        .header(AUTHORIZATION, format!("bearer {}", token))
        .json(&body)
        .send()
        .await?;

    Ok(response.json().await?)
}
