use crate::state::{action::Action, State};
use crate::ui::ui_action::UIAction;
use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};
use tokio::sync::mpsc::UnboundedSender;

pub trait Component {
    fn from_state(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    #[allow(unused_variables)]
    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        self
    }

    #[allow(unused_variables)]
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        Ok(None)
    }
}

pub trait Keyable {
    #[allow(unused_variables)]
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        Ok(None)
    }
}

pub trait StaticComponent<Props> {
    fn new(props: Props, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    #[allow(unused_variables)]
    fn move_with_props(self, props: Props) -> Self
    where
        Self: Sized,
    {
        self
    }
}

pub trait ComponentRender<Props> {
    fn render(&self, area: Rect, buf: &mut Buffer, props: Props);
}
