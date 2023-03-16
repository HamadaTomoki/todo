use anyhow::{Context, Ok, Result};
use clap::Parser;
use config::Config;
use notion::{ids::DatabaseId, models::search::DatabaseQuery, NotionApi};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "novum")]
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

// #[derive(Debug)]
// struct Task {
//     name: PropertyValue,
//     status: PropertyValue,
//     category: PropertyValue,
//     project: PropertyValue,
//     date: PropertyValue,
//     mind: PropertyValue,
// }

#[tokio::main]
async fn main() -> Result<()> {
    // Setup Notion API
    let opts = Opts::parse();
    let config = Config::builder()
        .add_source(config::File::with_name("Secrets"))
        .add_source(config::Environment::with_prefix("NOTION"))
        .build()?;
    let config: TodoConfig = config.try_deserialize().context("Failed to read config")?;
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

    // Use command
    match opts.command {
        SubCommand::List => list_tasks(notion_api, database_id).await,
        SubCommand::Add => add_task(notion_api),
    }
}

async fn list_tasks(notion_api: NotionApi, database_id: DatabaseId) -> Result<()> {
    let query = DatabaseQuery::default();
    let response = notion_api.query_database(database_id, query).await?;
    response.results.iter().for_each(|x| {
        x.properties
            .properties
            .iter()
            .for_each(|y| println!(" {}\n{:?}\n\n", y.0, y.1));
        println!("---------------------------------------------------------------------------------------------------------------------");
    });
    Ok(())
}

fn add_task(_notion_api: NotionApi) -> Result<()> {
    // let req =  PageCreateRequest { parent: notion::models::Parent::Page { page_id: () }}
    Ok(())
}
