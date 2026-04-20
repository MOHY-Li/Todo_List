use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub priority: Priority,
    pub category: String,
    #[serde(default)]
    pub date: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, Copy)]
pub enum Priority {
    #[default]
    Medium,
    Low,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "低"),
            Self::Medium => write!(f, "中"),
            Self::High => write!(f, "高"),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Copy)]
pub enum Tab {
    Yesterday,
    #[default]
    Today,
    Tomorrow,
}

impl Tab {
    pub const fn title(self) -> &'static str {
        match self {
            Self::Yesterday => "昨天",
            Self::Today => "今天",
            Self::Tomorrow => "明天",
        }
    }

    /// Tab button label with date, e.g. "今天 04/20"
    pub fn label(self) -> String {
        let date = self.target_date();
        format!("{} {}", self.title(), date.format("%m/%d"))
    }

    fn target_date(self) -> chrono::NaiveDate {
        let today = Local::now().date_naive();
        match self {
            Self::Yesterday => today - chrono::Duration::days(1),
            Self::Today => today,
            Self::Tomorrow => today + chrono::Duration::days(1),
        }
    }

    pub fn target_date_str(self) -> String {
        self.target_date().format("%Y-%m-%d").to_string()
    }

    pub fn filter_todos(self, todos: &[Todo]) -> Vec<&Todo> {
        let target = self.target_date_str();
        todos.iter().filter(|t| t.date == target).collect()
    }
}

/// Get today's date string in YYYY-MM-DD format.
#[allow(dead_code)]
pub fn today_str() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

/// Get today's date formatted for display, e.g. "2026/04/20 周日"
pub fn today_display() -> String {
    let today = Local::now().date_naive();
    let weekdays = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"];
    let wd = weekdays[today.weekday().num_days_from_sunday() as usize];
    format!("{} {}", today.format("%Y/%m/%d"), wd)
}

/// Return the three valid date strings: yesterday, today, tomorrow.
pub fn three_day_window() -> [String; 3] {
    [
        Tab::Yesterday.target_date_str(),
        Tab::Today.target_date_str(),
        Tab::Tomorrow.target_date_str(),
    ]
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
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    cats.sort();
    cats
}
