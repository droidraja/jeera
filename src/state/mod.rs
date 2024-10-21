use crate::api::models::JiraTask;




mod server;
pub mod action;
pub mod state_store;

#[derive(Default, Clone)]
pub struct State {
    pub current_sprint_tasks: Vec<JiraTask>
}


impl State  {
    fn set_current_sprint_tasks(self : &mut Self, current_sprint_tasks: Vec<JiraTask>) {
        self.current_sprint_tasks = current_sprint_tasks;
    }
}

