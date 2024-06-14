use twapi_v2::{api::get_2_users_me, oauth10a::OAuthAuthentication};

async fn execute_x() -> anyhow::Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    match get_2_users_me::Api::new().execute(&auth).await {
        Ok((response, _rate_limit)) => {
            println!("{:?}", response);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = execute_x().await?;

    Ok(())
}
