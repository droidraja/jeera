use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
};
use tokio::sync::mpsc::UnboundedSender;
use crate::state::{State, action::Action};
use anyhow::Result;

pub trait Component {
    fn new(state: &State, 
        action_tx: UnboundedSender<Action>
    ) -> Self
    where
        Self: Sized;
    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized;

    fn name(&self) -> &str;

    fn handle_key_event(&mut self, key: KeyEvent, on_enter: Option<Box<dyn FnMut(usize)>>) -> Result<()>;

}

pub trait ComponentRender<Props> {
    fn render(&self, area: Rect, buf: &mut Buffer, props: Props);
}
