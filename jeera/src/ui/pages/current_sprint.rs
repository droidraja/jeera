use crate::api::models::JiraTask;
use crate::state::action::Action;
use crate::state::State;

use crate::ui::components::{issue_detail::IssueDetail, issue_list::IssueList, Component};
use crate::ui::components::{issue_list, ComponentRender, Keyable, StaticComponent};
use crate::ui::ui_action::UIAction;
use anyhow::Ok;
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default, Debug)]
struct StateProps {
    issue_list: Vec<JiraTask>,
}

impl From<&State> for StateProps {
    fn from(value: &State) -> Self {
        Self {
            issue_list: value.current_sprint_tasks.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CurrentSprintPage {
    issue_list_comp: IssueList,
    issue_detail_comp: Option<IssueDetail>,
    props: StateProps,
    action_tx: UnboundedSender<Action>,
    selected_issue: Option<String>,
}

impl Component for CurrentSprintPage {
    fn move_with_state(self, state: &crate::state::State) -> Self
    where
        Self: Sized,
    {
        let props: StateProps = state.into();
        let mut new_issue_detail: Option<IssueDetail> = None;
        let mut new_selected_issue: Option<String> = None;

        if let Some(issue_key) = &self.selected_issue {
            let updated_issue = props.issue_list.iter().find(|item| item.key.eq(issue_key));
            if updated_issue.is_some() {
                let updated_issue = updated_issue.unwrap().clone();
                new_selected_issue = self.selected_issue;
                new_issue_detail = Some(
                    self.issue_detail_comp
                        .unwrap()
                        .move_with_props(updated_issue),
                );
            }
        }

        Self {
            issue_list_comp: self.issue_list_comp.move_with_state(state),
            props,
            selected_issue: new_selected_issue,
            issue_detail_comp: new_issue_detail,
            ..self
        }
    }

    fn from_state(
        state: &crate::state::State,
        action_tx: UnboundedSender<crate::state::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        let this: CurrentSprintPage = Self {
            issue_list_comp: IssueList::from_state(state, action_tx.clone()),
            issue_detail_comp: None,
            props: state.into(),
            action_tx,
            selected_issue: None,
        };
        // let _ = this.action_tx.send(Action::GetCurrentTasks);
        this
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> anyhow::Result<Option<UIAction>> {
        if key.code == crossterm::event::KeyCode::Esc {
            self.selected_issue = None;
            self.issue_detail_comp = None;
            return Ok(None);
        }

        if key.code == crossterm::event::KeyCode::Char('r') {
            let _ = self.action_tx.send(Action::GetCurrentTasks);
        }

        if let Some(ref mut issue_detail_comp) = self.issue_detail_comp {
            if let Some(action) = issue_detail_comp.handle_key_event(key)? {
                tracing::info!(
                    "Unhandled UI Action received from issue detail {:?}",
                    action
                );
                return Ok(None);
            }
        }

        if let Some(UIAction::ListItemClick(clicked_id)) =
            self.issue_list_comp.handle_key_event(key)?
        {
            tracing::info!("Received {} from issue_list", clicked_id);
            let issue = self.props.issue_list.get(clicked_id).unwrap();

            self.selected_issue = Some(issue.key.clone());
            self.issue_detail_comp =
                Some(IssueDetail::new(issue.to_owned(), self.action_tx.clone()));

            return Ok(None);
        }

        Ok(None)
    }
}

impl ComponentRender<()> for CurrentSprintPage {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _props: ()) {
        // tracing::info!("Is Selected {}",self.selected_issue.is_some());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut issue_list_area = &area;
        if let Some(issue_detail) = &self.issue_detail_comp {
            // tracing::info!("{}",selected_index);
            issue_detail.render(chunks[1], buf, ());

            issue_list_area = &chunks[0];
        }

        self.issue_list_comp.render(
            issue_list_area.to_owned(),
            buf,
            issue_list::RenderProps {
                issue_list: &self.props.issue_list,
            },
        );
    }
}
