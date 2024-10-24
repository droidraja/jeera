
use anyhow::Ok;

use crate::api::models::JiraTask;

use super::{Component, ComponentRender};

use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Widget, Wrap}
};



#[derive(Debug)]
pub struct IssueDetail {

}



impl Component for IssueDetail {
    fn new(_state: &crate::state::State, 
        _action_tx: tokio::sync::mpsc::UnboundedSender<crate::state::action::Action>
    ) -> Self
    where
        Self: Sized {
        IssueDetail{}
    }

    fn move_with_state(self, _state: &crate::state::State) -> Self
    where
        Self: Sized {
        self
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn handle_key_event(&mut self, _key: crossterm::event::KeyEvent) -> anyhow::Result<Option<crate::ui::ui_action::UIAction>> {
        Ok(None)
    }
}

pub struct RenderProps{
    pub task: JiraTask
}

impl ComponentRender<RenderProps> for IssueDetail {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, props: RenderProps) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);

        // Render title
        let title = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(&props.task.key, Style::default().fg(Color::Yellow)),
                Span::raw(" - "),
                Span::raw(&props.task.summary),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Issue"));
        title.render(chunks[0], buf);

        // Render status and priority
        let status_priority = Paragraph::new(vec![
            Line::from(vec![
                Span::raw("Status: "),
                Span::styled(&props.task.status, Style::default().fg(Color::Cyan)),
                Span::raw(" | Priority: "),
                Span::styled(&props.task.priority, Style::default().fg(Color::Magenta)),
            ]),
        ]);
        status_priority.render(chunks[1], buf);

        // Render other details
        let details = vec![
            Line::from(vec![Span::raw("Type: "), Span::raw(&props.task.issue_type)]),
            Line::from(vec![
                Span::raw("Assignee: "),
                Span::raw(props.task.asssignee.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Reporter: "),
                Span::raw(props.task.reported.as_deref().unwrap_or("Unknown")),
            ]),
            Line::from(vec![
                Span::raw("Team: "),
                Span::raw(props.task.team.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Parent: "),
                Span::raw(props.task.parent.as_deref().unwrap_or("None")),
            ]),
            Line::from(vec![
                Span::raw("Sprint: "),
                Span::raw(String::from("something")),
                Span::raw(props.task.sprint.as_ref().map_or(String::from("None"), |s| s.join(", "))),
            ]),
        ];

        let details_widget = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });
        details_widget.render(chunks[2], buf);
    }
}
