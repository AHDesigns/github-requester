#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kibana::get_pull_request_data().await?;

    Ok(())
}
