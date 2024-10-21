use std::collections::HashMap;

use super::{
    components::{Component, ComponentRender},
    pages::current_sprint::CurrentSprintPage,
};

const TABS: [&str; 2] = ["Current Sprint", "Assigned Tasks"];

use crate::state::{action::Action, State};
use anyhow::{Ok, Result};
use block::Title;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{block::Position, Block},
};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
#[derive(Debug)]
pub struct TabPage {
    tabs: Vec<String>,
    _pages: HashMap<String, String>,
    current_tab: usize,
    action_tx: UnboundedSender<Action>,
    pub current_sprint: CurrentSprintPage,
}

impl TabPage {
    fn get_default_tabs() -> Vec<String> {
        TABS.iter().map(|x| String::from(*x)).collect()
    }
}

impl TabPage {
    fn next_tab(self: &mut Self) {
        self.current_tab += 1;
        self.current_tab %= self.tabs.len();

        if self.current_tab == 0 {
            self.action_tx.send(Action::GetCurrentTasks).unwrap();
        }
    }
    fn prev_tab(self: &mut Self) {
        if self.current_tab == 0 {
            self.current_tab = self.tabs.len() - 1;
        } else {
            self.current_tab = self.current_tab.saturating_sub(1);
        }

        if self.current_tab == 0 {
            self.action_tx.send(Action::GetCurrentTasks).unwrap();
        }
    }
}

impl Component for TabPage {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let current_sprint = CurrentSprintPage::new(state, action_tx.clone());

        Self {
            tabs: Self::get_default_tabs(),
            _pages: HashMap::new(),
            current_tab: 0,
            action_tx,
            current_sprint,
        }
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            current_sprint: self.current_sprint.move_with_state(state),
            ..self
        }
    }

    fn name(&self) -> &str {
        "Tab Page"
    }

    
    fn handle_key_event<F>(&mut self, key: KeyEvent, _: Option<F>) -> Result<()>
    where 
        F: FnOnce(usize)
        {
        match key.code {
            KeyCode::Char('q') => {
                let _ = self.action_tx.send(Action::Exit);
            }
            KeyCode::Right => {
                self.next_tab();
            }
            KeyCode::Left => {
                self.prev_tab();
            }
            _ => {
                // todo!("pass other keycodes to the child pages to handle ")
            }
        };
        Ok(())
    }
}

impl ComponentRender<()> for TabPage {
    fn render(&self, area: Rect, buf: &mut Buffer, _props: ()) {
        let outer_box = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(0)])
            .margin(1)
            .split(area);

        let layout: std::rc::Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Fill(1)])
            .split(outer_box[0]);

        let title = Title::from("Personal Jira DashBoard".bold());
        let instructions = Title::from(Line::from(vec![
            " Previous Tab ".into(),
            "<Left>".blue().bold(),
            " Next Tab ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);
        block.render(area, buf);

        let _tabs = Tabs::new(self.tabs.clone())
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_set(border::THICK),
            )
            .select(self.current_tab)
            .divider(symbols::DOT)
            .padding("[", "]")
            .render(layout[0], buf);

        let _ = self.current_sprint.render(layout[1], buf, ());
    }
}
