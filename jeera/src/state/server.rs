use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use super::action::Action;
use crate::{
    api::JiraApi,
    state::{app_initializer, LoginState},
    termination::Interrupted,
};

pub struct MiddleWare {
    downstream_tx: UnboundedSender<Action>,
    jira_api: JiraApi,
}

impl MiddleWare {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let rxtx = mpsc::unbounded_channel::<Action>();
        let jira_api = JiraApi::new();
        (
            Self {
                jira_api,
                downstream_tx: rxtx.0,
            },
            rxtx.1,
        )
    }

    pub fn reload_api_config(&mut self) {
        self.jira_api.reload_config();
    }

    pub fn handle(&self, action: Action) -> anyhow::Result<()> {
        tracing::info!("Sending event to state store {:?}", action);
        self.downstream_tx.send(action)?;
        Ok(())
    }

    pub async fn main_loop(
        mut self,
        mut upstream_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                Some(action) = upstream_rx.recv() =>
                match action {
                    Action::GetCurrentTasks => {
                     tokio::spawn(self.jira_api.clone().get_current_tasks(self.downstream_tx.clone()));
                    },
                    Action::Exit => {
                        let _ = self.handle(action);
                        return Ok(());
                    },
                    Action::TransitionIssue(issue_key, transition_id) => {
                        let _ = self.handle(Action::TransitionIssueStarted);
                        tokio::spawn({
                            let jira_api = self.jira_api.clone();
                            let downstream_tx = self.downstream_tx.clone();
                            async move {
                                match jira_api.transition_issue(&issue_key, &transition_id).await {
                                    Ok(_) => {
                                        tracing::info!("transition issue successful");
                                        let _ = downstream_tx.send(Action::TransitionIssueFinished);
                                        tokio::spawn(jira_api.get_current_tasks(downstream_tx));
                                    },
                                    Err(err) => {
                                        tracing::info!("transition issue failed, {:#?}", err);
                                        let _ = downstream_tx.send(Action::TransitionIssueFailed);
                                    }
                                }
                            }
                        });
                    },
                    Action::Initialize => {
                        tracing::info!("initializing state store");
                        let status = app_initializer::try_login(self.downstream_tx.clone()).await;
                        if status == LoginState::LoggedIn {
                            tokio::spawn(self.jira_api.clone().get_current_tasks(self.downstream_tx.clone()));
                        }
                    },
                    Action::TryLogin { username, password, host } => {
                        let login_state = app_initializer::try_login_with_credentials(
                            self.downstream_tx.clone(),
                            username,
                            password,
                            host,
                        ).await;

                        if login_state == LoginState::LoggedIn {
                            self.reload_api_config();
                            tokio::spawn(self.jira_api.clone().get_current_tasks(self.downstream_tx.clone()));
                        }
                    },
                    _ => {
                        let _ = self.handle(action);
                    }
                },
                Ok(_interrupted) = interrupt_rx.recv() => {
                    break Ok(());
                }
            }
        }
    }
}
