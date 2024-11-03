use std::fmt::Display;

use super::{
    components::{
        background_tasks::BackgroundTasks, create_issue::CreateIssue, Component, ComponentRender,
    },
    pages::current_sprint::TasksPage,
    ui_action::UIAction,
};

use crate::state::{action::Action, State};
use anyhow::{Ok, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::Block,
};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum Tab {
    CurrentSprint,
    AllAssignedTasks,
    CreateIssue,
    Settings,
    EpicView,
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tab::CurrentSprint => "Current Sprint",
            Tab::AllAssignedTasks => "All Assigned Tasks",
            Tab::CreateIssue => "Create Issue",
            Tab::Settings => "Settings",
            Tab::EpicView => "Epic View",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct TabPage {
    tabs: Vec<Tab>,
    current_sprint: TasksPage,
    all_assigned_tasks: TasksPage,
    create_issue: Option<CreateIssue>,
    current_tab_index: usize,
    background_tasks_diplay: BackgroundTasks,
    action_tx: UnboundedSender<Action>,
    state: State,
}

impl TabPage {
    fn get_default_tabs() -> Vec<Tab> {
        vec![Tab::CurrentSprint, Tab::AllAssignedTasks]
    }
}

impl TabPage {
    fn next_tab(self: &mut Self) {
        self.current_tab_index = (self.current_tab_index + 1) % self.tabs.len();
    }

    fn prev_tab(self: &mut Self) {
        self.current_tab_index = (self.current_tab_index - 1) % self.tabs.len();
    }

    fn get_current_tab(&self) -> Tab {
        self.tabs[self.current_tab_index].clone()
    }

    fn push_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
        self.current_tab_index = self.tabs.len() - 1;
    }

    fn pop_tab(&mut self) {
        self.tabs.pop();
        self.current_tab_index = self.tabs.len() - 1;
    }

    fn get_tab_titles(&self) -> Vec<String> {
        self.tabs.iter().map(|x| x.to_string()).collect()
    }
}

impl Component for TabPage {
    fn from_state(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let current_sprint = TasksPage::new_current_sprint_page(state, action_tx.clone());
        let all_assigned_tasks = TasksPage::new_all_tasks_page(state, action_tx.clone());
        let background_tasks_diplay = BackgroundTasks::from_state(state, action_tx.clone());
        Self {
            tabs: Self::get_default_tabs(),
            background_tasks_diplay,
            action_tx: action_tx.clone(),
            current_sprint,
            all_assigned_tasks,
            create_issue: None,
            current_tab_index: 0,
            state: state.clone(),
        }
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            current_sprint: self.current_sprint.move_with_state(state),
            all_assigned_tasks: self.all_assigned_tasks.move_with_state(state),
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
                    self.push_tab(Tab::CreateIssue);
                } else {
                    self.create_issue = None;
                    self.pop_tab();
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
                match self.tabs[self.current_tab_index] {
                    Tab::CurrentSprint => self.current_sprint.handle_key_event(key)?,
                    Tab::AllAssignedTasks => self.all_assigned_tasks.handle_key_event(key)?,
                    Tab::CreateIssue => {
                        if let Some(action) =
                            self.create_issue.as_mut().unwrap().handle_key_event(key)?
                        {
                            if action == UIAction::CloseCreateIssue {
                                self.create_issue = None;
                                self.pop_tab();
                            }
                        }
                        None
                    }
                    _ => {
                        tracing::info!("unhandled tab {:?}", self.tabs[self.current_tab_index]);
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

        let title = Line::from("Personal Jira DashBoard").centered().bold();
        let instructions = Line::from(vec![
            " Previous Tab ".into(),
            "<Ctrl+Left>".blue().bold(),
            " Next Tab ".into(),
            "<Ctrl+Right>".blue().bold(),
            " Quit ".into(),
            "<Ctrl+c> ".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title_top(title)
            .title_bottom(instructions)
            .border_set(border::THICK);
        block.render(area, buf);

        let _tabs: () = Tabs::new(self.get_tab_titles())
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_set(border::THICK),
            )
            .select(self.current_tab_index as usize)
            .divider(symbols::DOT)
            .padding("[", "]")
            .render(layout[0], buf);

        match self.get_current_tab() {
            Tab::CurrentSprint => {
                let _ = self.current_sprint.render(layout[1], buf, ());
            }
            Tab::AllAssignedTasks => {
                let _ = self.all_assigned_tasks.render(layout[1], buf, ());
            }
            Tab::CreateIssue => self
                .create_issue
                .as_ref()
                .unwrap()
                .render(layout[1], buf, ()),
            _ => todo!(),
        }
        let _ = self.background_tasks_diplay.render(layout[2], buf, ());
    }
}
