use crate::api::models::JiraTask;
use crate::state::action::Action;
use crate::state::State;

use crate::ui::components::{issue_detail, issue_list, ComponentRender};
use crate::ui::components::{issue_list::IssueList, issue_detail::IssueDetail, Component};
use crate::ui::ui_action::UIAction;
use anyhow::Ok;
use tokio::sync::mpsc::UnboundedSender;
use ratatui::layout::{Layout, Constraint, Direction};

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
    issue_detail_comp: IssueDetail,
    props: StateProps,
    action_tx: UnboundedSender<Action>,
    selected_issue: Option<usize>,
}

impl Component for CurrentSprintPage {
    fn move_with_state(self, state: &crate::state::State) -> Self
    where
        Self: Sized {
        let props: StateProps = state.into(); 
        Self { 
            issue_list_comp: self.issue_list_comp.move_with_state(state),
            issue_detail_comp: self.issue_detail_comp.move_with_state(state),
            props,
            ..self
        }
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
            issue_detail_comp: IssueDetail::new(state, action_tx.clone()),
            props: state.into(),
            action_tx,
            selected_issue: None,
        };
        let _ = this.action_tx.send(Action::GetCurrentTasks);
        this
    }
    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<Option<UIAction>>
    {

        if key.code == crossterm::event::KeyCode::Esc {
            self.selected_issue = None;
            return Ok(None);
        }
        
        if let Some(UIAction::ListItemClick(clicked_id)) = self.issue_list_comp.handle_key_event(key)? {
            tracing::info!("Received {} from child", clicked_id); 
            self.selected_issue = Some(clicked_id);
            return Ok(None);
        }


        if self.selected_issue.is_some() {
            if let Some(action) = self.issue_detail_comp.handle_key_event(key)? {
                tracing::info!("Unhandled UI Action received from issue detail {:?}",action)
            }
        }

        Ok(None)
    }
}

impl ComponentRender<()> for CurrentSprintPage {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _props: ()) {
        // tracing::info!("Is Selected {}",self.selected_issue.is_some());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);
        
        let mut issue_list_area= &area;
        if let Some(selected_index) = self.selected_issue {
            // tracing::info!("{}",selected_index);
            if let Some(selected_task) = self.props.issue_list.get(selected_index) {
                self.issue_detail_comp.render(chunks[1], buf, issue_detail::RenderProps {
                    task: selected_task.clone(),
                });
            }
            issue_list_area = &chunks[0];
        } 

        self.issue_list_comp.render(issue_list_area.to_owned(), buf, issue_list::RenderProps{issue_list: &self.props.issue_list});
        
    }
}
