use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JiraTask {
    pub issue_type: String,
    pub summary: String,
    pub key: String,
    pub parent: Option<String>,
    pub sprint: Option<Vec<String>>,
    pub priority: String,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
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
        tracing::info!("{:#?}", value.clone());
        let fields = &value["fields"];
        let issue_type = fields["issuetype"]["name"].as_str().unwrap_or_default().to_owned();
        let summary = fields["summary"].as_str().unwrap_or_default().to_owned();
        let key = value["key"].as_str().unwrap_or_default().to_owned();
        let priority = fields["priority"]["name"].as_str().unwrap_or_default().to_owned();
        let status = fields["status"]["name"].as_str().unwrap_or_default().to_owned();

        let parent = fields["parent"]["key"].as_str().map(|s| s.to_owned());
        
        let sprint = fields["customfield_10020"]
            .as_array()
            .map(|sprints| sprints.iter().filter_map(|s| s["name"].as_str().map(|name| name.to_owned())).collect());

        let assignee = fields["assignee"]["displayName"].as_str().map(|s| s.to_owned());
        let reporter = fields["reporter"]["displayName"].as_str().map(|s| s.to_owned());
        
        let team = fields["customfield_10001"]
            .get("name")
            .and_then(|name| name.as_str())
            .map(|s| s.to_owned());

        Self {
            issue_type,
            summary,
            key,
            parent,
            sprint,
            priority,
            assignee,  
            reporter,
            status,
            team,
        }
    }
}
