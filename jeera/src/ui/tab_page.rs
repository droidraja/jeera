use std::collections::HashMap;

use super::{
    components::{
        background_tasks::BackgroundTasks, create_issue::CreateIssue, Component, ComponentRender,
    },
    pages::current_sprint::CurrentSprintPage,
    ui_action::UIAction,
};

const TABS: [&str; 2] = ["Current Sprint", "Assigned Tasks"];

use crate::state::{action::Action, State};
use anyhow::{Ok, Result};
use block::Title;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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

#[derive(Debug, PartialEq, Clone)]
enum CurrentTab {
    CurrentSprint,
    AssignedTasks,
    CreateIssue,
}

#[derive(Debug)]
pub struct TabPage {
    tabs: Vec<String>,
    _pages: HashMap<String, String>,
    current_tab: CurrentTab,
    background_tasks_diplay: BackgroundTasks,
    action_tx: UnboundedSender<Action>,
    pub current_sprint: CurrentSprintPage,
    create_issue: Option<CreateIssue>,
    state: State,
}

impl TabPage {
    fn get_default_tabs() -> Vec<String> {
        TABS.iter().map(|x| String::from(*x)).collect()
    }
}

impl TabPage {
    fn next_tab(self: &mut Self) {
        self.current_tab = match self.current_tab {
            CurrentTab::CurrentSprint => CurrentTab::AssignedTasks,
            CurrentTab::AssignedTasks => CurrentTab::CurrentSprint,
            CurrentTab::CreateIssue => CurrentTab::CurrentSprint,
        };
    }
    fn prev_tab(self: &mut Self) {
        self.current_tab = match self.current_tab {
            CurrentTab::CurrentSprint => CurrentTab::AssignedTasks,
            CurrentTab::AssignedTasks => CurrentTab::CurrentSprint,
            CurrentTab::CreateIssue => CurrentTab::CurrentSprint,
        };

        if self.current_tab == CurrentTab::CurrentSprint {
            self.action_tx.send(Action::GetCurrentTasks).unwrap();
        }
    }
}

impl Component for TabPage {
    fn from_state(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let current_sprint = CurrentSprintPage::from_state(state, action_tx.clone());

        Self {
            tabs: Self::get_default_tabs(),
            _pages: HashMap::new(),
            current_tab: CurrentTab::CurrentSprint,
            background_tasks_diplay: BackgroundTasks::from_state(state, action_tx.clone()),
            action_tx,
            current_sprint,
            create_issue: None,
            state: state.clone(),
        }
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            current_sprint: self.current_sprint.move_with_state(state),
            background_tasks_diplay: self.background_tasks_diplay.move_with_state(state),
            ..self
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = self.action_tx.send(Action::Exit);
            }
            KeyCode::Char('c') => {
                if self.create_issue.is_none() {
                    self.create_issue =
                        Some(CreateIssue::from_state(&self.state, self.action_tx.clone()));
                    self.current_tab = CurrentTab::CreateIssue;
                } else {
                    self.current_tab = CurrentTab::CurrentSprint;
                    self.create_issue = None;
                }
            }
            KeyCode::Right if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.next_tab();
            }
            KeyCode::Left if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.prev_tab();
            }
            _ => {
                // todo!("pass other keycodes to the child pages to handle ")
                match self.current_tab {
                    CurrentTab::CurrentSprint => self.current_sprint.handle_key_event(key)?,
                    CurrentTab::AssignedTasks => todo!(),
                    CurrentTab::CreateIssue => {
                        if let Some(action) =
                            self.create_issue.as_mut().unwrap().handle_key_event(key)?
                        {
                            if action == UIAction::CloseCreateIssue {
                                self.create_issue = None;
                                self.current_tab = CurrentTab::CurrentSprint;
                            }
                        }
                        None
                    }
                };
            }
        };
        Ok(None)
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
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(outer_box[0]);

        let title = Title::from("Personal Jira DashBoard".bold());
        let instructions = Title::from(Line::from(vec![
            " Previous Tab ".into(),
            "<Alt+Left>".blue().bold(),
            " Next Tab ".into(),
            "<Alt+Right>".blue().bold(),
            " Quit ".into(),
            "<Ctlr+c> ".blue().bold(),
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

        let _tabs: () = Tabs::new(self.tabs.clone())
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_set(border::THICK),
            )
            .select(self.current_tab.clone() as usize)
            .divider(symbols::DOT)
            .padding("[", "]")
            .render(layout[0], buf);

        match self.current_tab {
            CurrentTab::CurrentSprint => {
                let _ = self.current_sprint.render(layout[1], buf, ());
            }
            CurrentTab::AssignedTasks => todo!(),
            CurrentTab::CreateIssue => {
                self.create_issue
                    .as_ref()
                    .unwrap()
                    .render(layout[1], buf, ())
            }
        }
        let _ = self.background_tasks_diplay.render(layout[2], buf, ());
    }
}
