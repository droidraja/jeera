use crate::api::models::JiraTask;

use super::LoginState;

#[derive(Debug)]
pub enum Action {
    Exit,
    Initialize,
    LoginStatus(LoginState),
    TryLogin {
        username: String,
        password: String,
        host: String,
    },
    GetCurrentTasks,
    GetCurrentTasksStarted,
    GetCurrentTasksFailed,
    GetCurrentTasksFinished(Vec<JiraTask>),
    TransitionIssue(String, String),
    TransitionIssueStarted,
    TransitionIssueFailed,
    TransitionIssueFinished,
    CreateIssue {
        summary: String,
        description: String,
        issue_type: String,
    },
    CreateIssueStarted,
    CreateIssueFailed,
    CreateIssueFinished(JiraTask),
}
