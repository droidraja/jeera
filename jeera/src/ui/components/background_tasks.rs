use crate::state::action::Action;
use crate::state::{BgStatus, BgTask, State};
use ratatui::prelude::{Buffer, Rect};
use ratatui::text::Line;
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
    widgets::Widget,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, ComponentRender};

#[derive(Debug)]
pub struct BackgroundTasks {
    tasks: Vec<BgTask>,
}

impl Component for BackgroundTasks {
    fn from_state(state: &State, _action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let tasks = state.bg_tasks.values().cloned().collect();
        BackgroundTasks { tasks }
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        let tasks = state.bg_tasks.values().cloned().collect();
        BackgroundTasks { tasks }
    }
}

impl ComponentRender<()> for BackgroundTasks {
    fn render(&self, area: Rect, buf: &mut Buffer, _props: ()) {
        let task_lines: Vec<Span> = self
            .tasks
            .iter()
            .flat_map(|task| {
                let color = match task.status {
                    BgStatus::Running => Style::default().bg(Color::Gray).fg(Color::Black),
                    BgStatus::Finished(_) => Style::default().bg(Color::Green).fg(Color::Black),
                    BgStatus::Failed(_) => Style::default().bg(Color::Red).fg(Color::White),
                };
                vec![Span::styled(task.name.clone(), color), Span::raw(" ")]
            })
            .collect();

        let paragraph = Paragraph::new(vec![Line::from(task_lines)]);
        paragraph.render(area, buf);
    }
}
