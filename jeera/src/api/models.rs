use jira_cloud::models::{IssueBean, IssueTransition};
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
    pub transitions: Vec<IssueTransition>,
}

impl Display for JiraTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] - {}", self.key, self.summary)
    }
}

impl From<IssueBean> for JiraTask {
    fn from(issue: IssueBean) -> Self {
        let fields = issue.fields.as_ref().unwrap();

        Self {
            issue_type: fields
                .get("issuetype")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            summary: fields
                .get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            key: issue.key.clone().unwrap_or_default(),
            parent: fields
                .get("parent")
                .and_then(|v| v.get("key"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            sprint: fields
                .get("customfield_10020")
                .and_then(|v| v.as_array())
                .map(|sprints| {
                    sprints
                        .iter()
                        .filter_map(|s| {
                            s.get("name")
                                .and_then(|name| name.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect()
                }),
            priority: fields
                .get("priority")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            assignee: fields
                .get("assignee")
                .and_then(|v| v.get("displayName"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            reporter: fields
                .get("reporter")
                .and_then(|v| v.get("displayName"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            status: fields
                .get("status")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            team: fields
                .get("customfield_10001")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            transitions: issue.transitions.unwrap_or_default(),
        }
    }
}

impl From<serde_json::Value> for JiraTask {
    fn from(value: serde_json::Value) -> Self {
        tracing::info!("{:#?}", value.clone());
        let fields = &value["fields"];
        let issue_type = fields["issuetype"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let summary = fields["summary"].as_str().unwrap_or_default().to_owned();
        let key = value["key"].as_str().unwrap_or_default().to_owned();
        let priority = fields["priority"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let status = fields["status"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();

        let parent = fields["parent"]["key"].as_str().map(|s| s.to_owned());

        let sprint = fields["customfield_10020"].as_array().map(|sprints| {
            sprints
                .iter()
                .filter_map(|s| s["name"].as_str().map(|name| name.to_owned()))
                .collect()
        });

        let assignee = fields["assignee"]["displayName"]
            .as_str()
            .map(|s| s.to_owned());
        let reporter = fields["reporter"]["displayName"]
            .as_str()
            .map(|s| s.to_owned());

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
            transitions: Vec::default(),
        }
    }
}
