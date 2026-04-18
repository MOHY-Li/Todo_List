use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub priority: Priority,
    pub category: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Copy)]
pub enum Priority {
    #[default]
    Medium,
    Low,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "低"),
            Priority::Medium => write!(f, "中"),
            Priority::High => write!(f, "高"),
        }
    }
}

#[derive(Default, Clone, PartialEq, Copy)]
pub enum Tab {
    Yesterday,
    #[default]
    Today,
    Tomorrow,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Yesterday => "昨天",
            Tab::Today => "今天",
            Tab::Tomorrow => "明天",
        }
    }

    pub fn filter_todos<'a>(&self, todos: &'a [Todo]) -> Vec<&'a Todo> {
        let now = Local::now();
        let today = now.date_naive();
        let yesterday = today - chrono::Duration::days(1);
        let tomorrow = today + chrono::Duration::days(1);

        match self {
            Tab::Yesterday => todos
                .iter()
                .filter(|t| {
                    let Some(dt) = Local.timestamp_opt(t.created_at as i64, 0).single() else {
                        return false;
                    };
                    dt.date_naive() == yesterday
                })
                .collect(),
            Tab::Today => todos
                .iter()
                .filter(|t| {
                    let Some(dt) = Local.timestamp_opt(t.created_at as i64, 0).single() else {
                        return false;
                    };
                    dt.date_naive() == today
                })
                .collect(),
            Tab::Tomorrow => todos
                .iter()
                .filter(|t| {
                    let Some(dt) = Local.timestamp_opt(t.created_at as i64, 0).single() else {
                        return false;
                    };
                    dt.date_naive() == tomorrow
                })
                .collect(),
        }
    }
}

pub fn count_by_status(todos: &[Todo]) -> (usize, usize) {
    let active = todos.iter().filter(|t| !t.completed).count();
    let completed = todos.len() - active;
    (active, completed)
}

pub fn get_categories(todos: &[Todo]) -> Vec<String> {
    let mut cats: Vec<String> = todos
        .iter()
        .map(|t| t.category.clone())
        .filter(|c| !c.is_empty())
        .collect();
    cats.sort();
    cats.dedup();
    cats
}
