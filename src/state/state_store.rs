
use super::server::MiddleWare;

use tokio::sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    };
use crate::{Interrupted, Terminator};

use super::{action::Action, State};

pub struct StateStore {
    state_tx: UnboundedSender<State>,
}

impl StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State>();

        (StateStore { state_tx }, state_rx)
    }
}

impl StateStore {
    pub async fn main_loop(
        self,
        mut terminator: Terminator,
        action_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {

        let mut state: State = State::default();
        
        let (middleware,mut middleware_rx) = MiddleWare::new();
        
        tokio::spawn(middleware.main_loop(action_rx));
        // the initial state once
        self.state_tx.send(state.clone())?;

        let result = loop {
            tokio::select! {
                Some(action) = middleware_rx.recv() => match action {
                    Action::Exit => {
                        let _ = terminator.terminate(Interrupted::UserInt);
    
                        break Interrupted::UserInt;
                    },
                    Action::GetCurrentTasksFinished(issues) => {
                        state.set_current_sprint_tasks(issues);
                    },
                    //capture other actions to silence errors during dev 
                    unhandled_action => {
                        tracing::info!("Unhandled Action {:?}", unhandled_action);
                    }
                },
                // Catch and handle interrupt signal to gracefully shutdown
                Ok(interrupted) = interrupt_rx.recv() => {
                    break interrupted;
                }
            }

            self.state_tx.send(state.clone())?;
        };
        

        Ok(result)
    }
}