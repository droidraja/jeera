use std::sync::Arc;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    widgets::{List, ListState, StatefulWidget},
    style::{Style, Color},
    text::Span,
};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::Mutex;

use super::{ComponentRender, Keyable, StaticComponent};
use crate::ui::ui_action::UIAction;
use crate::state::action::Action;

#[derive(Debug)]
pub struct DropdownMenu {
    list_state: Arc<Mutex<ListState>>,
    props: Props
}

#[derive(Debug)]
pub struct Props {
    pub options:  Vec<String>,
    pub title:  String,
}

impl StaticComponent<Props> for DropdownMenu {
    fn new(
        props: Props,
        _action_tx: UnboundedSender<Action>
    ) -> Self
    where
        Self: Sized,
    {
        DropdownMenu {
            list_state: Arc::new(Mutex::new(ListState::default())),
            props
        }
    }

}

impl Keyable for DropdownMenu {
    
    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<Option<UIAction>> {
        match key.code {
            KeyCode::Up => {
                self.list_state.lock().unwrap().select_previous();
            }
            KeyCode::Down => {
                self.list_state.lock().unwrap().select_next();
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.lock().unwrap().selected() {
                    return Ok(Some(UIAction::DropdownItemSelected(index)));
                }
            }
            _ => {}
        }
        Ok(None)
    }
}


impl ComponentRender<()> for DropdownMenu {
    fn render(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _props: (),
    ) {
        let items: Vec<Span> = self.props.options.iter()
            .map(|option| Span::raw(option.clone()))
            .collect();

        let mut list_state = self.list_state.lock().unwrap();

        let list = List::new(items)
            .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black))
            .highlight_symbol("> ")
            .block(ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title(self.props.title.clone()));

        StatefulWidget::render(list, area, buf, &mut list_state);
    }
}
