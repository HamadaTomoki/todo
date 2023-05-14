use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use config::Config;
use model::{
    config::{Opts, SubCommand, TodoConfig},
    data::create_page_req,
};
use notion::{
    ids::DatabaseId,
    models::{
        properties::PropertyValue, search::DatabaseQuery, PageCreateRequest, Parent, Properties,
    },
    NotionApi,
};
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    // Set notion config
    let opts = Opts::parse();
    let config = Config::builder()
        .add_source(config::File::with_name("Secrets"))
        .add_source(config::Environment::with_prefix("NOTION"))
        .build()?;
    let config: TodoConfig = config.try_deserialize().context("Failed to read config")?;

    // Get API Token
    let notion_api = NotionApi::new(
        config
            .clone()
            .api_token
            .expect("Failed to read NOTION_API_TOKEN"),
    )?;

    // Get database instance by database_id
    let database_id = config
        .clone()
        .database_id
        .expect("Failed to read NOTION_DATABASE_ID");

    // Use command
    match opts.command {
        SubCommand::List => list_tasks(notion_api, database_id).await,
        SubCommand::Add { task_name } => {
            if let Some(task) = task_name {
                add_task_to_inbox(notion_api, database_id, &task).await
            } else {
                Ok(())
            }
        }
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

async fn add_task_to_inbox(
    notion_api: NotionApi,
    database_id: DatabaseId,
    task_name: &str,
) -> Result<()> {
    let properties: HashMap<String, PropertyValue> = create_page_req(task_name);
    let properties = Properties { properties };
    let req = PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
    };
    notion_api.create_page(req).await?;
    Ok(())
}
