use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub period: String, // e.g., "daily", "monthly"
    pub data: Vec<ReportItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportItem {
    pub label: String,
    pub value: f64,
}
