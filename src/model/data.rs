use std::{collections::HashMap, str::FromStr};

use notion::{
    ids::PropertyId,
    models::{
        properties::{Color, PropertyValue, SelectedValue},
        text::{RichText, RichTextCommon, Text},
    },
};
use uuid::Uuid;

pub fn create_page_req(task_name: &str) -> HashMap<String, PropertyValue> {
    vec![
        (
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
                        content: task_name.into(),
                        link: None,
                    },
                }],
            },
        ),
        (
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
        (
            "タスク種別".into(),
            PropertyValue::Select {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                select: None,
            },
        ),
        (
            "プロジェクト".to_string(),
            PropertyValue::Relation {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                relation: Some(vec![]),
            },
        ),
        (
            "実施予定日".into(),
            PropertyValue::Date {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                date: None,
            },
        ),
        (
            "気持ち".into(),
            PropertyValue::Text {
                id: PropertyId::from_str(&Uuid::new_v4().to_string()).unwrap(),
                rich_text: vec![],
            },
        ),
    ]
    .into_iter()
    .collect()
}
