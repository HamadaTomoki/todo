use clap::Parser;
use notion::ids::DatabaseId;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "novum")]
pub struct Opts {
    #[clap(subcommand)]
    pub command: SubCommand,
}
#[derive(Parser, Debug)]
pub enum SubCommand {
    List,
    Add { task_name: Option<String> },
}
#[derive(Deserialize, Serialize, Clone)]
pub struct TodoConfig {
    pub api_token: Option<String>,
    pub database_id: Option<DatabaseId>,
}
