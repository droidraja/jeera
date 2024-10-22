pub mod models;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tokio::sync::mpsc::UnboundedSender;
use directories::BaseDirs;
use std::fs;
use serde::{Serialize, Deserialize};

use anyhow::Result;
use models::JiraTask;
use reqwest::{self, Client};
use serde_json::Value;

use crate::state::action::Action;

const API_TOKEN: &str = "";
const HOST: &str = "soupcop.atlassian.net";
const EMAIL: &str = "srajasudhan@gmail.com";

#[derive(Serialize,Deserialize,Clone)]
struct JiraConfig {
    email: String,
    api_token: String,
    host: String,
}

#[derive(Clone)]
pub struct JiraApi {
    config: JiraConfig,
    client: reqwest::Client,
}

impl JiraApi {
    pub fn new() -> Result<Self> {
        let config = Self::load_config()?;
        let client = Client::new();
        
        Ok(Self {
            config,
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

    pub async fn get_current_tasks(self, action_tx: UnboundedSender<Action>)  {
        let headers = self.get_headers().unwrap();
        let url = self.get_url("search");
        let jql = "assignee=currentUser() AND resolution=Unresolved ORDER BY priority DESC";
        let _ = action_tx.send(Action::GetCurrentTasksStarted);

        let response = self
            .client
            .get(url)
            .headers(headers)
            .query(&[("jql", jql)])
            .send()
            .await;
        let response = match response {
            Ok(response) => {
                response
            },
            Err(e) => {
                tracing::error!("Jira Api Failed {:#}",e);
                let _ = action_tx.send(Action::GetCurrentTasksFailed);
                return;
            }
        };

        let response = match response.error_for_status() {
            Ok(response) => response,
            Err(_) => {
                let _ = action_tx.send(Action::GetCurrentTasksFailed);
                return;
            }
        };



        // let text = response.text().await?;
        // tracing::info!("{}",text);
        
        let data:Value = response
            .json()
            .await.unwrap();

        let jira_issues :Vec<JiraTask> = data["issues"].as_array().unwrap().iter().map(|x| JiraTask::from(x.clone())).collect();
        let _ = action_tx.send(Action::GetCurrentTasksFinished(jira_issues));
    }
}
