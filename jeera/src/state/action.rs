use crate::api::models::JiraTask;



#[derive(Debug)]
pub enum Action {
    Exit,
    GetCurrentTasks,
    GetCurrentTasksStarted,
    GetCurrentTasksFailed,
    GetCurrentTasksFinished(Vec<JiraTask>),
    TransitionIssue(String, String), // (issue_key, transition_id)
    TransitionIssueStarted,
    TransitionIssueFailed,
    TransitionIssueFinished,
}
