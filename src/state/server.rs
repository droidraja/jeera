use anyhow::Ok;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};


use super::action::Action;
use crate::api::JiraApi;

pub struct MiddleWare {
    downstream_tx : UnboundedSender<Action>,
    jira_api: JiraApi
}



impl MiddleWare {
    pub fn new() -> (Self, UnboundedReceiver<Action>){
        let rxtx =  mpsc::unbounded_channel::<Action>();
        let jira_api = JiraApi::new().unwrap();
        (Self{jira_api,downstream_tx:rxtx.0},rxtx.1)
    }

    pub fn handle(&self,action : Action) -> anyhow::Result<()> {
        tracing::info!("Sending event to state store {:?}", action);
        self.downstream_tx.send(action)?;
        Ok(())
    }

    pub async fn main_loop(self,
        mut upstream_rx: UnboundedReceiver<Action>) -> anyhow::Result<()> {
        tracing::info!("Testing logging");
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
                    _ => {
                        let _ = self.handle(action);
                    }
                }
            }
        }
    }
}