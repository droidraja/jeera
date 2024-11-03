use crate::api::models::JiraTask;

use super::LoginState;
use jeera_derive::IsStarted;

#[derive(Debug, Clone)]
pub enum APILifeCycle<T, U> {
    Start(T),
    Started,
    Finished(U),
    Failed,
}

impl<T, U> APILifeCycle<T, U> {
    pub fn is_started(&self) -> bool {
        matches!(self, Self::Started)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished(_))
    }
}

pub trait IsStarted {
    fn is_started(&self) -> bool;
    fn is_finished(&self) -> bool;
    fn is_failed(&self) -> bool;
    fn name(&self) -> String;
}

#[allow(dead_code)]
#[derive(Debug, Clone, IsStarted)]
pub enum APICall {
    GetAllTasks(APILifeCycle<(), Vec<JiraTask>>),
    GetCurrentTasks(APILifeCycle<(), Vec<JiraTask>>),
    TransitionIssue(APILifeCycle<(String, String), ()>),
    CreateIssue(APILifeCycle<(), JiraTask>),
}

impl APICall {
    pub fn transition_issue(issue_key: String, transition_id: String) -> Self {
        Self::TransitionIssue(APILifeCycle::Start((issue_key, transition_id)))
    }
}

impl From<APICall> for Action {
    fn from(call: APICall) -> Self {
        Action::API(call)
    }
}

#[derive(Debug)]
pub enum Action {
    API(APICall),
    Exit,
    Initialize,
    LoginStatus(LoginState),
    TryLogin {
        username: String,
        password: String,
        host: String,
    },
}
