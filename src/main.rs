use anyhow::{anyhow, Context, Result};
use notion::{ids::DatabaseId, NotionApi};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TodoConfig {
    api_token: Option<String>,
    task_database_id: Option<DatabaseId>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::default()
        .with_merged(config::File::with_name("Secrets"))
        .unwrap_or_default()
        .with_merged(config::Environment::with_prefix("NOTION"))?;

    let config: TodoConfig = config.try_into().context("Failed to read config.")?;
    let notin_api = NotionApi::new(
        std::env::var("NOTION_API_TOKEN")
            .or(config.api_token.ok_or(anyhow!("No api token from config")))
            .context(
                "No Notion API token found in either the environment variable \
                        `NOTION_API_TOKEN` or the config file!",
            )?,
    )?;
    Ok(())
}
