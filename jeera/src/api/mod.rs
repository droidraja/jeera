pub mod models;

use jira_cloud::models::User;
use jira_teams::models::PublicApiTeam;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::action::Action;
use crate::state::config::Config;
use anyhow::Result;
use jira_cloud::apis::issue_search_api;
use jira_cloud::apis::issues_api::do_transition;
use jira_cloud::apis::{configuration::Configuration as JiraAPIConfig, myself_api};
use jira_teams::apis::configuration::Configuration as TeamsAPIConfig;
use jira_teams::apis::teams_public_api_api::query_teams;
use models::JiraTask;
use reqwest::{self, Client};
use serde_json::{json, Value};

#[derive(Clone)]
#[allow(dead_code)]
pub struct JiraApi {
    config: Config,
    teams_config: TeamsAPIConfig,
    jira_config: JiraAPIConfig,
    client: reqwest::Client,
}

impl From<&Config> for JiraAPIConfig {
    fn from(value: &Config) -> Self {
        let mut config = JiraAPIConfig::new();
        config.base_path = value.host.clone();
        config.basic_auth = Some((value.email.clone(), Some(value.api_token.clone())));
        config
    }
}

impl From<&Config> for TeamsAPIConfig {
    fn from(value: &Config) -> Self {
        let mut config = TeamsAPIConfig::new();
        config.base_path = value.host.clone();
        config.basic_auth = Some((value.email.clone(), Some(value.api_token.clone())));
        config
    }
}

impl From<&Config> for JiraApi {
    fn from(config: &Config) -> Self {
        let client = Client::new();
        let teams_config = TeamsAPIConfig::from(config);
        let jira_config = JiraAPIConfig::from(config);

        Self {
            config: config.clone(),
            teams_config,
            jira_config,
            client,
        }
    }
}

impl JiraApi {
    pub fn new() -> Self {
        let config = Config::load_config();
        if let Ok(config) = config {
            Self::from(&config)
        } else {
            Self::from(&Config::default())
        }
    }

    pub fn reload_config(&mut self) {
        self.config = Config::load_config().unwrap();
        self.jira_config = JiraAPIConfig::from(&self.config);
        self.teams_config = TeamsAPIConfig::from(&self.config);
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

    pub async fn get_current_user(&self) -> Result<User> {
        myself_api::get_current_user(&self.jira_config, None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get current user: {:#}", e))
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

        let jql = "assignee=currentUser() AND Sprint in openSprints() AND resolution=Unresolved ORDER BY priority DESC";
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
