use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

// use ratatui::widgets::block::Block;
use ratatui::widgets::Widget;

use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::{Input, TextArea};

use crate::{
    state::{action::Action, State},
    ui::{
        components::{Component, ComponentRender},
        ui_action::UIAction,
    },
};

#[derive(Debug)]
pub struct CreateIssue {
    summary_input: TextArea<'static>,
    description_input: TextArea<'static>,
    issue_type_input: TextArea<'static>,
    active_input: ActiveInput,
    action_tx: UnboundedSender<Action>,
}

#[derive(Debug, PartialEq)]
enum ActiveInput {
    Summary,
    Description,
    IssueType,
}

impl Component for CreateIssue {
    fn from_state(_state: &State, action_tx: UnboundedSender<Action>) -> Self {
        let mut summary_input = TextArea::default();
        summary_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Summary")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        summary_input.set_placeholder_text("Enter issue summary");
        // summary_input.set_max_rows(1);

        let mut description_input = TextArea::default();
        description_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        description_input.set_placeholder_text("Enter issue description");

        let mut issue_type_input = TextArea::default();
        issue_type_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Issue Type")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        issue_type_input.set_placeholder_text("Task, Bug, Story, etc.");
        // issue_type_input.set_max_rows(1);

        Self {
            summary_input,
            description_input,
            issue_type_input,
            active_input: ActiveInput::Summary,
            action_tx,
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        match key.code {
            KeyCode::Tab => {
                self.active_input = match self.active_input {
                    ActiveInput::Summary => ActiveInput::Description,
                    ActiveInput::Description => ActiveInput::IssueType,
                    ActiveInput::IssueType => ActiveInput::Summary,
                };
                Ok(None)
            }
            KeyCode::Enter => {
                if self.active_input != ActiveInput::Description {
                    return Ok(None);
                }
                let active_input = match self.active_input {
                    ActiveInput::Summary => &mut self.summary_input,
                    ActiveInput::Description => &mut self.description_input,
                    ActiveInput::IssueType => &mut self.issue_type_input,
                };
                active_input.input(Input::from(key));
                Ok(None)
            }
            KeyCode::Esc => Ok(Some(UIAction::CloseCreateIssue)),
            KeyCode::Char('s')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                let summary = self.summary_input.lines()[0].clone();
                let description = self.description_input.lines().join("\n");
                let issue_type = self.issue_type_input.lines()[0].clone();

                if !summary.is_empty() && !issue_type.is_empty() {
                    self.action_tx
                        .send(Action::CreateIssue {
                            summary,
                            description,
                            issue_type,
                        })
                        .unwrap();
                    Ok(Some(UIAction::CloseCreateIssue))
                } else {
                    Ok(None)
                }
            }
            _ => {
                let active_input = match self.active_input {
                    ActiveInput::Summary => &mut self.summary_input,
                    ActiveInput::Description => &mut self.description_input,
                    ActiveInput::IssueType => &mut self.issue_type_input,
                };
                active_input.input(Input::from(key));
                Ok(None)
            }
        }
    }
}

impl ComponentRender<()> for CreateIssue {
    fn render(&self, area: Rect, buf: &mut Buffer, _props: ()) {
        // let popup_area = centered_rect(60, 60, area);

        // // Clear the background
        // Clear::default().render(popup_area, buf);

        //Using it as a tab page and not as a popup for now
        let popup_area = area;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Summary
                Constraint::Min(5),    // Description
                Constraint::Length(3), // Issue Type
                Constraint::Length(2), // Help text
            ])
            .split(popup_area);

        // Render inputs
        self.summary_input.render(chunks[0], buf);
        self.description_input.render(chunks[1], buf);
        self.issue_type_input.render(chunks[2], buf);

        // Render help text
        let help_text = Paragraph::new(Line::from(
            "Press Tab to switch fields, Ctrl+S to save, Esc to cancel",
        ))
        .style(Style::default().fg(Color::Gray));
        help_text.render(chunks[3], buf);
    }
}
