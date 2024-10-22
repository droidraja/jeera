use std::sync::Arc;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::widgets::{List, ListState, StatefulWidget};
use std::sync::Mutex;

use crate::api::models::JiraTask;

use super::{Component, ComponentRender};

use crate::ui::ui_action::UIAction;

#[derive(Default, Debug)]
pub struct IssueList {
    list_state: Arc<Mutex<ListState>>,
}

impl Component for IssueList {
    fn new(
        _: &crate::state::State,
        _: tokio::sync::mpsc::UnboundedSender<crate::state::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        IssueList {list_state:Arc::new(Mutex::new(ListState::default()))}
    }

    fn move_with_state(self, _: &crate::state::State) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn name(&self) -> &str {
        "Issue List"
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<Option<UIAction>>  {
        match key.code {
            KeyCode::Up => {
                self.list_state.lock().unwrap().select_previous();
            }
            KeyCode::Down => {
                self.list_state.lock().unwrap().select_next();
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.lock().unwrap().selected() {
                    return Ok(Some(UIAction::ListItemClick(index)))
                }                 
            }
            _ => {}
        }
        Ok(None)
    }
}

pub struct RenderProps<'a> {
    pub issue_list: &'a Vec<JiraTask>,
}

impl ComponentRender<RenderProps<'_>> for IssueList {
    fn render(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        props: RenderProps,
    ) {
        let titles: Vec<String> = props.issue_list.iter().map(|x| x.to_string()).collect();
        
        let mut list_state = self.list_state.lock().unwrap();


        let list = List::new(titles)
            .highlight_symbol("> ");

        StatefulWidget::render(list, area, buf, &mut list_state);

    }
}
