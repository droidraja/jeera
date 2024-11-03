use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::Context;

use crossterm::{
    event::{
        DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event, EventStream,
        KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver},
};
use tokio_stream::StreamExt;

use crate::{
    state::{action::Action, State},
    ui::components::{Component, ComponentRender},
    Interrupted,
};

use super::router_component::RouterComponent;

const RENDERING_TICK_RATE: Duration = Duration::from_millis(1000);

pub struct UILoop {
    action_tx: mpsc::UnboundedSender<Action>,
}

impl UILoop {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        (Self { action_tx }, action_rx)
    }

    pub async fn main_loop(
        self,
        mut state_rx: UnboundedReceiver<State>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        self.action_tx.send(Action::Initialize).unwrap();
        // consume the first state to initialize the ui app
        let mut app_router = {
            let state = state_rx.recv().await.unwrap();
            RouterComponent::from_state(&state, self.action_tx.clone())
        };

        let mut terminal = setup_terminal()?;
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);
        let mut crossterm_events = EventStream::new();

        let result: anyhow::Result<Interrupted> = loop {
            tokio::select! {
                // Tick to terminate the select every N milliseconds
                _ = ticker.tick() => (),
                // Catch and handle crossterm events
               maybe_event = crossterm_events.next() => match maybe_event {
                    Some(Ok(Event::Key(key))) if key.kind == KeyEventKind::Press  => {
                        app_router.handle_key_event(key)?;
                    },
                    Some(Ok(Event::FocusGained)) => {
                        tracing::info!("Focus gained");
                    }
                    Some(Ok(Event::FocusLost)) => {
                        tracing::info!("Focus lost");
                    }
                    None => break Ok(Interrupted::UserInt),
                    _ => (),
                },
                // Handle state updates
                Some(state) = state_rx.recv() => {
                    app_router = app_router.move_with_state(&state);
                },
                // Catch and handle interrupt signal to gracefully shutdown
                Ok(interrupted) = interrupt_rx.recv() => {
                    break Ok(interrupted);
                }
            }

            if let Err(err) = terminal
                .draw(|frame| app_router.render(frame.area(), frame.buffer_mut(), ()))
                .context("could not render to the terminal")
            {
                break Err(err);
            }
        };

        restore_terminal(&mut terminal)?;

        result
    }
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableFocusChange
    )?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(terminal.show_cursor()?)
}
