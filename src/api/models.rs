use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JiraTask {
    pub issue_type: String,
    pub summary: String,
    pub key: String,
    pub parent: Option<String>,
    pub sprint: Option<Vec<String>>,
    pub priority: String,
    pub asssignee: Option<String>,
    pub reported: Option<String>,
    pub status: String,
    pub team: Option<String>,
}

impl Display for JiraTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] - {}", self.key, self.summary)
    }
}

impl From<serde_json::Value> for JiraTask {
    fn from(value: serde_json::Value) -> Self {
        let fields = &value["fields"];
        let issue_type = fields["issuetype"]["name"].as_str().unwrap().to_owned();
        let summary = fields["summary"].as_str().unwrap().to_owned();
        let key = value["key"].as_str().unwrap().to_owned();
        let priority = fields["priority"]["name"].as_str().unwrap().to_owned();
        let status = fields["status"]["name"].as_str().unwrap().to_owned();

        Self {
            issue_type,
            summary,
            key,
            parent: None,
            sprint: None,
            priority,
            asssignee: None,
            reported: None,
            status,
            team: None,
        }
    }
}
