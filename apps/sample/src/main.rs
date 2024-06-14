use mockito::{Mock, Server, ServerGuard};
use serde_json::json;
use twapi_v2::{api::{get_2_users_me, TwapiOptions}, oauth10a::OAuthAuthentication};

async fn execute_x(twapi_options: Option<TwapiOptions>) -> anyhow::Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let mut api = get_2_users_me::Api::new();
    if let Some(twapi_options) = twapi_options {
        api = api.twapi_options(twapi_options);
    }
    match api.execute(&auth).await 
    {
        Ok((response, _rate_limit)) => {
            println!("{:?}", response);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

async fn make_mock() -> (ServerGuard, Mock) {
    let mut server = Server::new_async().await;
    let json = json!({
        "data": {
            "id": "123",
            "name": "モック",
            "username": "mock",
        }
    });
    let mock = server
        .mock("GET", "/2/users/me")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json.to_string())
        .create_async()
        .await;
    (server, mock)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // モック無し
    let _ = execute_x(None).await?;

    // モックあり
    let (server, mock) = make_mock().await;
    let twapi_options = Some(TwapiOptions {
        prefix_url: Some(server.url()),
        ..Default::default()
    });
    let _ = execute_x(twapi_options).await?;
    mock.assert();
    Ok(())
}
