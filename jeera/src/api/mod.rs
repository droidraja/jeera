pub mod models;

use directories::BaseDirs;
use jira_teams::models::PublicApiTeam;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::sync::mpsc::UnboundedSender;

use crate::state::action::Action;
use anyhow::Result;
use jira_cloud::apis::configuration::Configuration as JiraAPIConfig;
use jira_cloud::apis::issue_search_api;
use jira_cloud::apis::issues_api::do_transition;
use jira_teams::apis::configuration::Configuration as TeamsAPIConfig;
use jira_teams::apis::teams_public_api_api::query_teams;
use models::JiraTask;
use reqwest::{self, Client};
use serde_json::{json, Value};

const API_TOKEN: &str = "";
const HOST: &str = "soupcop.atlassian.net";
const EMAIL: &str = "srajasudhan@gmail.com";

#[derive(Serialize, Deserialize, Clone)]
struct JiraConfig {
    email: String,
    api_token: String,
    host: String,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct JiraApi {
    config: JiraConfig,
    teams_config: TeamsAPIConfig,
    jira_config: JiraAPIConfig,
    client: reqwest::Client,
}

impl From<&JiraConfig> for JiraAPIConfig {
    fn from(value: &JiraConfig) -> Self {
        let mut config = JiraAPIConfig::new();
        config.base_path = format!("https://{}", value.host);
        config.basic_auth = Some((value.email.clone(), Some(value.api_token.clone())));
        config
    }
}

impl From<&JiraConfig> for TeamsAPIConfig {
    fn from(value: &JiraConfig) -> Self {
        let mut config = TeamsAPIConfig::new();
        config.base_path = format!("https://{}", value.host);
        config.basic_auth = Some((value.email.clone(), Some(value.api_token.clone())));
        config
    }
}

#[allow(dead_code)]
impl JiraApi {
    pub fn new() -> Result<Self> {
        let config = Self::load_config()?;
        let client = Client::new();
        let teams_config = TeamsAPIConfig::from(&config);
        let jira_config = JiraAPIConfig::from(&config);

        Ok(Self {
            config,
            teams_config,
            jira_config,
            client,
        })
    }

    fn load_config() -> Result<JiraConfig> {
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.home_dir();
            let config_path = config_dir.join(".jeera").join("config.json");

            if config_path.exists() {
                let config_str = fs::read_to_string(config_path)?;
                let config: JiraConfig = serde_json::from_str(&config_str)?;
                return Ok(config);
            }
        }

        // Fallback to default values if config file is not found
        Ok(JiraConfig {
            email: EMAIL.into(),
            api_token: API_TOKEN.into(),
            host: HOST.into(),
        })
    }

    pub fn get_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let auth = base64::encode(format!("{}:{}", self.config.email, self.config.api_token));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", auth))?,
        );
        Ok(headers)
    }

    pub fn get_url(&self, query: &str) -> String {
        format!("https://{}/rest/api/2/{}", self.config.host, query)
    }

    pub async fn get_org_id(&self) -> Result<String> {
        let url = format!("https://{}/gateway/api/graphql", self.config.host);
        let graphql_query =
            "query example($host: String!) {tenantContexts(hostNames:[$host]) {orgId}}";
        let request_body = json!({
            "query":graphql_query,
            "variables": {
                "host":self.config.host
            }
        });
        let response = self
            .client
            .post(url)
            .headers(self.get_headers()?)
            .json(&request_body)
            .send()
            .await?;

        let json_resp: Value = response.json().await?;
        let org_id = json_resp["data"]["tenantContexts"][0]["orgId"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to extract orgId from response"))?
            .to_string();

        Ok(org_id)
    }

    pub async fn get_teams(
        &self,
        _action_tx: UnboundedSender<Action>,
    ) -> Result<Vec<PublicApiTeam>> {
        let org_id = self.get_org_id().await?;
        tracing::info!("Org Id {}", org_id);
        let response = query_teams(&self.teams_config, &org_id, None, None, None).await?;
        tracing::info!("{:#?}", response.entities);
        Ok(response.entities)
    }

    pub async fn get_current_tasks(self, action_tx: UnboundedSender<Action>) {
        let _ = action_tx.send(Action::GetCurrentTasksStarted);

        let jql = "assignee=currentUser() AND resolution=Unresolved ORDER BY priority DESC";
        let expand = "editmetadata,transitions";

        match issue_search_api::search_for_issues_using_jql(
            &self.jira_config,
            Some(jql),
            None,
            Some(50),
            Some("strict"),
            None,
            Some(expand),
            None,
            None,
            None,
        )
        .await
        {
            Ok(search_results) => {
                let issues: Vec<JiraTask> = search_results
                    .issues
                    .unwrap_or_default()
                    .into_iter()
                    .map(JiraTask::from)
                    .collect();
                let _ = action_tx.send(Action::GetCurrentTasksFinished(issues));
            }
            Err(e) => {
                tracing::error!("Jira Api Failed {:#}", e);
                let _ = action_tx.send(Action::GetCurrentTasksFailed);
            }
        }
    }

    pub async fn transition_issue(&self, issue_key: &str, transition_id: &str) -> Result<()> {
        let transition_object = jira_cloud::models::IssueUpdateDetails {
            transition: Some(jira_cloud::models::IssueTransition {
                id: Some(transition_id.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        do_transition(&self.jira_config, issue_key, transition_object).await?;
        Ok(())
    }
}
