use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::{Input, TextArea};

use crate::{
    state::{action::Action, LoginState, State},
    ui::{
        components::{Component, ComponentRender},
        ui_action::UIAction,
    },
};
use tui_big_text::{BigText, PixelSize};

#[derive(Debug, PartialEq)]
enum ActiveInput {
    Email,
    Host,
    ApiToken,
}

struct LoginPageProps {
    login_state: LoginState,
}

impl From<&State> for LoginPageProps {
    fn from(state: &State) -> Self {
        Self {
            login_state: state.login_state.clone(),
        }
    }
}

pub struct LoginPage {
    props: LoginPageProps,
    email_input: TextArea<'static>,
    host_input: TextArea<'static>,
    api_token_input: TextArea<'static>,
    active_input: ActiveInput,
    action_tx: UnboundedSender<Action>,
}

impl LoginPage {
    fn validate_inputs(&self) -> Option<String> {
        // Validate email
        let email = self.email_input.lines()[0].trim();
        if !email.contains('@') || !email.contains('.') {
            return Some("Invalid email address".to_string());
        }

        // Validate host
        let host = self.host_input.lines()[0].trim();
        if !host.starts_with("https://") || !host.ends_with(".atlassian.net") {
            return Some("Host should be of format https://something.atlassian.net".to_string());
        }

        // Validate API token (just check if it's not empty)
        let api_token = self.api_token_input.lines()[0].trim();
        if api_token.is_empty() {
            return Some("API token cannot be empty".to_string());
        }
        None
    }

    fn set_active_input(&mut self) {
        if self.active_input == ActiveInput::Email {
            activate(&mut self.email_input);
        } else {
            inactivate(&mut self.email_input);
        }

        if self.active_input == ActiveInput::Host {
            activate(&mut self.host_input);
        } else {
            inactivate(&mut self.host_input);
        }

        if self.active_input == ActiveInput::ApiToken {
            activate(&mut self.api_token_input);
        } else {
            inactivate(&mut self.api_token_input);
        }
    }
}

fn inactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    let mut block = textarea.block().unwrap().clone();
    block = block.border_style(Style::default().fg(Color::Reset));
    textarea.set_block(block);
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut block = textarea.block().unwrap().clone();
    block = block.border_style(Style::default().fg(Color::Yellow));
    textarea.set_block(block);
}

impl Component for LoginPage {
    fn from_state(state: &State, action_tx: UnboundedSender<Action>) -> Self {
        let mut email_input = TextArea::default();
        email_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Email")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        email_input.set_placeholder_text("Enter your email");

        let mut host_input = TextArea::default();
        host_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Host")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        host_input.set_placeholder_text("https://your-domain.atlassian.net");

        let mut api_token_input = TextArea::default();
        api_token_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("API Token")
                .title_style(Style::default().fg(Color::Yellow)),
        );
        api_token_input.set_placeholder_text("Enter your API token");

        let mut this = Self {
            props: LoginPageProps::from(state),
            email_input,
            host_input,
            api_token_input,
            active_input: ActiveInput::Email,
            action_tx,
        };

        this.set_active_input();
        this
    }

    fn move_with_state(self, state: &State) -> Self {
        Self {
            props: LoginPageProps::from(state),
            email_input: self.email_input,
            host_input: self.host_input,
            api_token_input: self.api_token_input,
            active_input: self.active_input,
            action_tx: self.action_tx,
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        // Don't handle input if logging in
        if matches!(self.props.login_state, LoginState::LoggingIn) {
            return Ok(None);
        }

        let to_return = match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_tx.send(Action::Exit).unwrap();
                Ok(None)
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.active_input = match self.active_input {
                        ActiveInput::Email => ActiveInput::ApiToken,
                        ActiveInput::Host => ActiveInput::Email,
                        ActiveInput::ApiToken => ActiveInput::Host,
                    };
                } else {
                    self.active_input = match self.active_input {
                        ActiveInput::Email => ActiveInput::Host,
                        ActiveInput::Host => ActiveInput::ApiToken,
                        ActiveInput::ApiToken => ActiveInput::Email,
                    };
                }
                Ok(None)
            }
            KeyCode::Enter => {
                if let None = self.validate_inputs() {
                    self.action_tx
                        .send(Action::TryLogin {
                            username: self.email_input.lines()[0].trim().to_string(),
                            password: self.api_token_input.lines()[0].trim().to_string(),
                            host: self.host_input.lines()[0].trim().to_string(),
                        })
                        .unwrap();
                }
                Ok(None)
            }
            _ => {
                let active_input = match self.active_input {
                    ActiveInput::Email => &mut self.email_input,
                    ActiveInput::Host => &mut self.host_input,
                    ActiveInput::ApiToken => &mut self.api_token_input,
                };
                active_input.input(Input::from(key));
                Ok(None)
            }
        };

        self.set_active_input();
        to_return
    }
}
impl ComponentRender<()> for LoginPage {
    fn render(&self, area: Rect, buf: &mut ratatui::prelude::Buffer, _props: ()) {
        let width_padding = (area.width as f64 * 0.1) as u16;
        let height_padding = (area.height as f64 * 0.1) as u16;
        let padded_area = Rect::new(
            area.x + width_padding,
            area.y + height_padding,
            area.width - 2 * width_padding,
            area.height - 2 * height_padding,
        );
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // ASCII art
                Constraint::Length(3), // Email
                Constraint::Length(3), // Host
                Constraint::Length(3), // API Token
                Constraint::Length(2), // Status/Error message
                Constraint::Min(0),    // Remaining space
            ])
            .split(padded_area);

        // Render ASCII art
        let big_text = BigText::builder()
            .centered()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::new().fg(Color::Yellow))
            .lines(vec!["Jeera".into()])
            .build();

        big_text.render(chunks[0], buf);
        // Render inputs
        self.email_input.render(chunks[1], buf);
        self.host_input.render(chunks[2], buf);
        self.api_token_input.render(chunks[3], buf);

        // Render status/error message
        let status_message = match self.props.login_state {
            LoginState::NotLoggedIn => {
                if let Some(error) = self.validate_inputs() {
                    error
                } else {
                    "Press Enter to login".to_string()
                }
            }
            LoginState::LoggingIn => "Logging in...".to_string(),
            LoginState::LoginFailed => "Login failed. Please try again.".to_string(),
            LoginState::LoggedIn => "Logged in successfully".to_string(),
        };

        let status_style = match self.props.login_state {
            LoginState::LoginFailed => Style::default().fg(Color::Red),
            LoginState::LoggedIn => Style::default().fg(Color::Green),
            _ => Style::default().fg(Color::Gray),
        };

        Paragraph::new(Line::from(status_message))
            .style(status_style)
            .render(chunks[4], buf);
    }
}
