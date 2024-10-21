use std::any::Any;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::widgets::{List, ListState, StatefulWidget, Widget};

use crate::api::models::JiraTask;

use super::{Component, ComponentRender};

#[derive(Default, Debug)]
pub struct IssueList {
    pub selected_index: usize,
    list_state: ListState,
}

impl Component for IssueList {
    fn new(
        _: &crate::state::State,
        _: tokio::sync::mpsc::UnboundedSender<crate::state::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        IssueList { selected_index: 0, list_state:ListState::default()}
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

    fn handle_key_event(&mut self, key: KeyEvent, on_enter: Option<Box<dyn FnMut(usize)>>) -> anyhow::Result<()>  {
        
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                self.selected_index += 1; // For simplicity, we're not checking the upper bound
            }
            KeyCode::Enter => {
                if let Some(mut callback) = on_enter {
                    callback(self.selected_index);
                }
            }
            _ => {}
        }
        Ok(())
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

        let mut list_state = self.list_state.clone();

        let list = List::new(titles)
            .highlight_symbol("> ");

        StatefulWidget::render(list, area, buf, &mut list_state);

    }
}
