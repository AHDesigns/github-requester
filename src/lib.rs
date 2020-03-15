#[macro_use]
extern crate serde_json;

mod request;
use request::github::{self, GetPrArgs};

pub async fn get_pull_request_data() -> Result<(), Box<dyn std::error::Error>> {
    let config = GetPrArgs {
        name: "skyport-graphql".to_string(),
        owner: "sky-uk".to_string(),
    };

    let body = json!({ "query": &github::query(config) });

    let res = request::make_request(body).await?;

    println!("res: {:#?}", res);

    Ok(())
}
