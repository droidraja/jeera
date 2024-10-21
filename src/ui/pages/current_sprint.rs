
use crate::api::models::JiraTask;
use crate::state::action::Action;
use crate::state::State;

use crate::ui::components::{issue_list, ComponentRender};
use crate::ui::components::{issue_list::IssueList, Component};
use crate::ui::ui_action::UIAction;
use anyhow::Ok;
use tokio::sync::mpsc::UnboundedSender;



#[derive(Default, Debug)]
struct StateProps {
    issue_list: Vec<JiraTask>
}

impl From<&State> for StateProps {
    fn from(value: &State) -> Self {
        Self { issue_list: value.current_sprint_tasks.clone() }
    }
}

#[derive(Debug)]
pub struct CurrentSprintPage {
    issue_list_comp: IssueList,
    props: StateProps,
    action_tx: UnboundedSender<Action>,
    selected_issue: Option<usize>,
}



impl Component for CurrentSprintPage {
    
    fn move_with_state(self, state: &crate::state::State) -> Self
    where
        Self: Sized {
        let props: StateProps = state.into(); 
        Self { issue_list_comp: self.issue_list_comp.move_with_state(state), props, ..self }
    }
    
    fn name(&self) -> &str {
        "Current Sprint"
    }
    
    
    fn new(state: &crate::state::State, 
        action_tx: UnboundedSender<crate::state::action::Action>
    ) -> Self
    where
        Self: Sized {
        let this = Self {
            issue_list_comp: IssueList::new(state, action_tx.clone()),
            props: state.into(),
            action_tx,
            selected_issue: None,
        };
        let _ = this.action_tx.send(Action::GetCurrentTasks);
        this
    }
    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<Option<UIAction>>
    {
        
        if let Some(UIAction::ListItemClick(clicked_id)) = self.issue_list_comp.handle_key_event(key).unwrap() {
            tracing::info!("Received {} from child", clicked_id); 
            self.selected_issue = Some(clicked_id);
        }

        Ok(None)
    }
    
}


impl ComponentRender<()> for CurrentSprintPage {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _props: ()) {
        self.issue_list_comp.render(area, buf, issue_list::RenderProps{issue_list: &self.props.issue_list});
    }
}
