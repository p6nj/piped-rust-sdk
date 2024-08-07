use std::{borrow::Cow, env};

use piped::PipedClient;
use reqwest::Client;

#[tokio::main]
async fn main() -> piped::Result<()> {
    Ok({
        let client = Client::new();
        let piped = PipedClient::new(
            &client,
            env::var("INSTANCE")
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed("https://pipedapi.kavin.rocks")),
        );
        dbg!(
            piped
                .playlist_from_id("PLvVUc0cy2yXgVI8Dod5Uck-yiXoRAw_yX")
                .await?
                .name
        );
    })
}
