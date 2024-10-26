use anyhow::Ok;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};
use tokio::sync::mpsc::UnboundedSender;

use crate::api::models::JiraTask;

use super::component::Keyable;
use super::dropdown_menu::{DropdownMenu, Props};
use super::{ComponentRender, StaticComponent};
use crate::state::action::Action;
use crate::ui::ui_action::UIAction;

#[derive(Debug)]
pub struct IssueDetail {
    dropdown: Option<DropdownMenu>,
    task: JiraTask,
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
}

impl StaticComponent<JiraTask> for IssueDetail {
    fn new(jira_task: JiraTask, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let task = jira_task;
        IssueDetail {
            dropdown: None,
            task,
            action_tx,
        }
    }

    fn move_with_props(self, task: JiraTask) -> Self {
        Self { task, ..self }
    }
}

impl Keyable for IssueDetail {
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> anyhow::Result<Option<UIAction>> {
        match key.code {
            crossterm::event::KeyCode::Char('t') => {
                if self.dropdown.is_none() {
                    let transitions = &self.task.transitions;
                    let transition_options: Vec<String> = transitions
                        .into_iter()
                        .map(|t| t.name.clone().unwrap_or_default())
                        .collect();

                    let props = Props {
                        options: transition_options,
                        title: "Transitions".to_string(),
                    };
                    self.dropdown = Some(DropdownMenu::new(props, self.action_tx.clone()));
                } else {
                    self.dropdown = None;
                }
                Ok(None)
            }

            _ => {
                if let Some(dropdown) = &mut self.dropdown {
                    if let Some(UIAction::DropdownItemSelected(index)) =
                        dropdown.handle_key_event(key)?
                    {
                        self.dropdown = None;
                        let selected_transition = self.task.transitions.get(index).unwrap();
                        let transition_id = selected_transition.id.as_ref().unwrap().to_owned();
                        let issue_key = self.task.key.clone();
                        self.action_tx
                            .send(Action::TransitionIssue(issue_key, transition_id))
                            .unwrap();
                        return Ok(None);
                    }
                }
                Ok(None)
            }
        }
    }
}

impl ComponentRender<()> for IssueDetail {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _props: ()) {
        let available_width = area.width.saturating_sub(2) as usize;
        let wrapped_title = textwrap::wrap(&self.task.summary, available_width);
        let title_height = (wrapped_title.len() + 2) as u16;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(title_height),
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);

        let mut title_lines = vec![Line::from(vec![
            Span::styled(&self.task.key, Style::default().fg(Color::Yellow)),
            Span::raw(" - "),
            Span::raw(wrapped_title[0].clone()),
        ])];

        // Add the rest of the wrapped title lines, if any
        if wrapped_title.len() > 1 {
            title_lines.extend(
                wrapped_title[1..]
                    .iter()
                    .map(|line| Line::from(line.as_ref())),
            );
        }

        // Render title
        let title = Paragraph::new(title_lines)
            .block(Block::default().borders(Borders::ALL).title("Issue"));
        title.render(chunks[0], buf);

        // Render status and priority
        let status_priority = Paragraph::new(vec![Line::from(vec![
            Span::raw("Status: "),
            Span::styled(&self.task.status, Style::default().fg(Color::Cyan)),
            Span::raw(" | Priority: "),
            Span::styled(&self.task.priority, Style::default().fg(Color::Magenta)),
        ])]);
        status_priority.render(chunks[1], buf);

        // Render other details
        let details = vec![
            Line::from(vec![Span::raw("Type: "), Span::raw(&self.task.issue_type)]),
            Line::from(vec![
                Span::raw("Assignee: "),
                Span::raw(self.task.assignee.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Reporter: "),
                Span::raw(self.task.reporter.as_deref().unwrap_or("Unknown")),
            ]),
            Line::from(vec![
                Span::raw("Team: "),
                Span::raw(self.task.team.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Parent: "),
                Span::raw(self.task.parent.as_deref().unwrap_or("None")),
            ]),
            Line::from(vec![
                Span::raw("Sprint: "),
                Span::raw(
                    self.task
                        .sprint
                        .as_ref()
                        .map_or(String::from("None"), |s| s.join(", ")),
                ),
            ]),
        ];

        let details_widget = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });
        details_widget.render(chunks[2], buf);

        // Render the dropdown menu if it's active
        if let Some(dropdown) = &self.dropdown {
            let dropdown_height = (self.task.transitions.len() + 2) as u16;
            let dropdown_area =
                ratatui::layout::Rect::new(area.x + 2, area.y + 2, 30, dropdown_height);
            // Clear the render area before rendering the dropdown
            Clear::default().render(dropdown_area, buf);

            dropdown.render(dropdown_area, buf, ());
        }
    }
}
