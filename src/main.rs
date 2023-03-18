use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::{Context, Ok, Result};
use chrono::NaiveDate;
use clap::Parser;
use config::Config;
use notion::{
    ids::{AsIdentifier, DatabaseId, Identifier, PageId, PropertyId},
    models::{
        properties::{
            Color, DateOrDateTime, DateValue, PropertyValue, Relation, RelationValue, SelectedValue,
        },
        search::DatabaseQuery,
        text::{RichText, RichTextCommon, Text},
        Page, PageCreateRequest, Parent, Properties,
    },
    NotionApi,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: PropertyValue,
    status: PropertyValue,
    category: PropertyValue,
    project: PropertyValue,
    scheduled_date: PropertyValue,
    mind: PropertyValue,
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            name => write!(f, "名前"),
            status => write!(f, "ステータス"),
            category => write!(f, "タスク種別"),
            project => write!(f, "プロジェクト"),
            scheduled_date => write!(f, "実施予定日"),
            mind => write!(f, "気持ち"),
        }
    }
}

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
        SubCommand::Add => add_task(notion_api, database_id).await,
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

async fn add_task(notion_api: NotionApi, database_id: DatabaseId) -> Result<()> {
    let task = Task {
        name: PropertyValue::Title {
            id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
            title: vec![RichText::Text {
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
        status: PropertyValue::Status {
            id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
            status: Some(SelectedValue {
                id: None,
                name: Some("".into()),
                color: Color::Default,
            }),
        },
        category: PropertyValue::Select {
            id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
            select: Some(SelectedValue {
                id: None,
                name: Some("".into()),
                color: Color::Default,
            }),
        },
        project: PropertyValue::Relation {
            id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
            relation: None,
        },
        scheduled_date: PropertyValue::Date {
            id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
            date: Some(DateValue {
                start: DateOrDateTime::Date {
                    0: NaiveDate::default(),
                },
                end: None,
                time_zone: None,
            }),
        },
        mind: PropertyValue::Text {
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
    };
    let mut properties: HashMap<String, PropertyValue> = HashMap::new();
    // properties.insert("source".into());
    let properties = Properties { properties };
    let req = PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
    };
    Ok(())
}
