use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use config::Config;
use notion::{ids::DatabaseId, models::Database, NotionApi};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jake Swenson")]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}
#[derive(Parser, Debug)]
enum SubCommand {
    List,
    Add,
}
#[derive(Deserialize, Serialize, Clone)]
struct TodoConfig {
    api_token: Option<String>,
    database_id: Option<DatabaseId>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup Notion API
    let opts = Opts::parse();
    let config = config::Config::builder()
        .add_source(config::File::with_name("Secrets"))
        .add_source(config::Environment::with_prefix("NOTION"))
        .build()?;
    let config: TodoConfig = config.try_deserialize().context("")?;
    let notion_api = NotionApi::new(
        config
            .clone()
            .api_token
            .expect("Failed to read NOTION_API_TOKEN"),
    )?;

    // Get database instance from database_id
    let database_id = config
        .clone()
        .database_id
        .expect("Failed to read NOTION_DATABASE_ID");
    let database: Database = notion_api.get_database(database_id).await?;

    // Use command
    match opts.command {
        SubCommand::List => list_tasks(database),
        SubCommand::Add => add_task(database),
    }
}

fn list_tasks(db: Database) -> Result<()> {
    Ok(())
}
fn add_task(db: Database) -> Result<()> {
    Ok(())
}
