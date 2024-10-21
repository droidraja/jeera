pub mod models;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION}, Response, Url
};
use tokio::sync::mpsc::UnboundedSender;
use std::{collections::HashMap, fs::read};

use anyhow::{ Result};
use models::JiraTask;
use reqwest::{self, Client};
use serde_json::Value;

use crate::state::action::Action;

const API_TOKEN: &str = "";
const HOST: &str = "soupcop.atlassian.net";
const EMAIL: &str = "srajasudhan@gmail.com";

#[derive(Clone)]
pub struct JiraApi {
    email: String,
    api_token: String,
    host: String,
    client: reqwest::Client,
}

impl JiraApi {
    pub fn new() -> Self {
        let _client = Client::new();
        Self {
            email: EMAIL.into(),
            api_token: API_TOKEN.into(),
            host: HOST.into(),
            client: _client,
        }
    }

    pub fn get_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let auth = base64::encode(format!("{}:{}", self.email, self.api_token));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", auth))?,
        );
        Ok(headers)
    }

    pub fn get_url(&self, query: &str) -> String {
        format!("https://{}/rest/api/2/{}", self.host, query)
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
