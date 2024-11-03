use std::time::Duration;

use super::server::MiddleWare;

use crate::{
    state::action::{APICall, APILifeCycle},
    Interrupted, Terminator,
};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use super::{action::Action, action::IsStarted, State};

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

        let (middleware, mut middleware_rx) = MiddleWare::new();

        tokio::spawn(middleware.main_loop(action_rx, interrupt_rx.resubscribe()));
        let mut ticker = tokio::time::interval(Duration::from_secs(1));

        // the initial state once
        self.state_tx.send(state.clone())?;

        let result = loop {
            tokio::select! {
                Some(action) = middleware_rx.recv() => match action {
                    Action::Exit => {
                        let _ = terminator.terminate(Interrupted::UserInt);

                        break Interrupted::UserInt;
                    },
                    Action::LoginStatus(login_state) => {
                        state.set_login_state(login_state);
                    },
                    Action::API(call)  => {
                        if call.is_started() {
                            state.add_bg_task(call.name());
                        } else if call.is_finished() {
                            state.succeed_bg_task(call.name());
                        } else if call.is_failed() {
                            state.fail_bg_task(call.name());
                        }
                        match call {
                            APICall::TransitionIssue(life_cycle) => {
                                tracing::info!("GetAllTasks {:?}", life_cycle);
                            },
                            APICall::GetAllTasks(APILifeCycle::Finished(result)) => {
                                state.set_all_assigned_tasks(result);
                            },
                            APICall::GetCurrentTasks(APILifeCycle::Finished(result)) => {
                                state.set_current_sprint_tasks(result);
                            },
                            _ => {
                                tracing::info!("Unhandled API call in state_store {:?}", call);
                            }
                        };


                    },
                    //capture other actions to silence errors during dev
                    unhandled_action => {
                        tracing::info!("Unhandled Action {:?}", unhandled_action);
                    }

                },

                // Tick to terminate the select every N milliseconds
                _ = ticker.tick() => {
                    state.tick_timer();
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
