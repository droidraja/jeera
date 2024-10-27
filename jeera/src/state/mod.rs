use std::collections::HashMap;

use crate::api::models::JiraTask;

pub mod action;
mod server;
pub mod state_store;

#[derive(Clone, Debug)]
pub enum BgStatus {
    Running,
    Failed(std::time::SystemTime),
    Finished(std::time::SystemTime),
}

impl Default for BgStatus {
    fn default() -> Self {
        BgStatus::Running
    }
}

#[derive(Default, Clone, Debug)]
pub struct BgTask {
    pub name: String,
    pub status: BgStatus,
}

#[derive(Default, Clone)]
pub struct State {
    pub current_sprint_tasks: Vec<JiraTask>,
    pub bg_tasks: HashMap<String, BgTask>,
}

impl State {
    fn set_current_sprint_tasks(self: &mut Self, current_sprint_tasks: Vec<JiraTask>) {
        self.current_sprint_tasks = current_sprint_tasks;
    }

    fn add_bg_task(&mut self, name: String) {
        let bg_task = BgTask {
            name,
            status: BgStatus::Running,
        };
        self.bg_tasks.insert(bg_task.name.clone(), bg_task);
    }

    fn fail_bg_task(&mut self, name: String) {
        if let Some(task) = self.bg_tasks.get_mut(&name) {
            task.status = BgStatus::Failed(std::time::SystemTime::now())
        }
    }

    fn succeed_bg_task(&mut self, name: String) {
        if let Some(task) = self.bg_tasks.get_mut(&name) {
            task.status = BgStatus::Finished(std::time::SystemTime::now())
        }
    }

    fn evict_bg_tasks(&mut self) -> Option<usize> {
        let now = std::time::SystemTime::now();
        let mut dropped_count: usize = 0;

        self.bg_tasks.retain(|_, task| {
            match task.status {
                BgStatus::Failed(time) | BgStatus::Finished(time)
                    if now.duration_since(time).unwrap_or_default()
                        >= std::time::Duration::from_secs(4) =>
                {
                    dropped_count += 1;
                    false // Remove this task
                }
                _ => true, // Keep this task
            }
        });

        if dropped_count > 0 {
            Some(dropped_count)
        } else {
            None
        }
    }

    fn tick_timer(&mut self) {
        self.evict_bg_tasks();
    }
}