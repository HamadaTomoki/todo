use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Ok, Result};
use chrono::Utc;
use clap::Parser;
use config::Config;
use notion::{
    ids::{DatabaseId, PropertyId},
    models::{
        properties::{Color, DateOrDateTime, DateValue, PropertyValue, SelectedValue},
        search::DatabaseQuery,
        text::{RichText, RichTextCommon, Text},
        PageCreateRequest, Parent, Properties,
    },
    NotionApi,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: (String, PropertyValue),
    status: (String, PropertyValue),
    category: (String, PropertyValue),
    project: (String, PropertyValue),
    scheduled_date: (String, PropertyValue),
    mind: (String, PropertyValue),
}
// impl Default for Task {
//     fn default() -> Self {
//         Task {
//             name: "".into(),
//             status: "未着手".into(),
//             category: "".into(),
//             project: "".into(),
//             scheduled_date: "".into(),
//             mind: "".into(),
//         }
//     }
// }
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "novum")]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}
#[derive(Parser, Debug)]
enum SubCommand {
    List,
    Add { task_name: Option<String> },
}
#[derive(Deserialize, Serialize, Clone)]
struct TodoConfig {
    api_token: Option<String>,
    database_id: Option<DatabaseId>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set notion confik
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
                add_task(notion_api, database_id, task).await
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

async fn add_task(notion_api: NotionApi, database_id: DatabaseId, task_name: String) -> Result<()> {
    let task = Task {
        name: (
            "名前".into(),
            PropertyValue::Title {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                title: vec![RichText::Text {
                    rich_text: RichTextCommon {
                        plain_text: "".into(),
                        href: None,
                        annotations: None,
                    },
                    text: Text {
                        content: task_name,
                        link: None,
                    },
                }],
            },
        ),
        status: (
            "ステータス".into(),
            PropertyValue::Status {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                status: Some(SelectedValue {
                    id: None,
                    name: Some("未着手".into()),
                    color: Color::Default,
                }),
            },
        ),
        category: (
            "タスク種別".into(),
            PropertyValue::Select {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                select: None,
            },
        ),
        project: (
            "プロジェクト".to_string(),
            PropertyValue::Relation {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                relation: Some(vec![]),
            },
        ),
        scheduled_date: (
            "実施予定日".into(),
            PropertyValue::Date {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                date: Some(DateValue {
                    start: DateOrDateTime::Date {
                        0: Utc::now().date_naive(),
                    },
                    end: None,
                    time_zone: None,
                }),
            },
        ),
        mind: (
            "気持ち".into(),
            PropertyValue::Text {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                rich_text: vec![RichText::Text {
                    rich_text: RichTextCommon {
                        plain_text: "".into(),
                        href: None,
                        annotations: None,
                    },
                    text: Text {
                        content: "".into(),
                        link: None,
                    },
                }],
            },
        ),
    };
    let mut properties: HashMap<String, PropertyValue> = HashMap::new();
    properties.insert(task.name.0, task.name.1);
    properties.insert(task.status.0, task.status.1);
    properties.insert(task.category.0, task.category.1);
    properties.insert(task.project.0, task.project.1);
    properties.insert(task.scheduled_date.0, task.scheduled_date.1);
    properties.insert(task.mind.0, task.mind.1);
    let properties = Properties { properties };
    let req = PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
    };
    notion_api.create_page(req).await?;
    Ok(())
}
