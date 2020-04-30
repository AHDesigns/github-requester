use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    gitprs::get_pull_request_data().await?;

    Ok(())
}
