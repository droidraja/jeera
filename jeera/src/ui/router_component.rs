use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use super::{
    components::{Component, ComponentRender},
    pages::login_page::LoginPage,
    tab_page::TabPage,
    ui_action::UIAction,
};
use crate::state::{action::Action, LoginState, State};

struct RouterProps {
    login_state: LoginState,
}

impl From<&State> for RouterProps {
    fn from(state: &State) -> Self {
        Self {
            login_state: state.login_state.clone(),
        }
    }
}

pub struct RouterComponent {
    props: RouterProps,
    action_tx: UnboundedSender<Action>,
    login_page: LoginPage,
    tab_page: TabPage,
}

impl Component for RouterComponent {
    fn from_state(state: &State, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            props: RouterProps::from(state),
            action_tx: action_tx.clone(),
            login_page: LoginPage::from_state(state, action_tx.clone()),
            tab_page: TabPage::from_state(state, action_tx),
        }
    }

    fn move_with_state(self, state: &State) -> Self {
        Self {
            props: RouterProps::from(state),
            action_tx: self.action_tx,
            login_page: self.login_page.move_with_state(state),
            tab_page: self.tab_page.move_with_state(state),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<UIAction>> {
        match self.props.login_state {
            LoginState::LoggedIn => self.tab_page.handle_key_event(key),
            LoginState::NotLoggedIn | LoginState::LoggingIn | LoginState::LoginFailed => {
                self.login_page.handle_key_event(key)
            }
        }
    }
}

impl ComponentRender<()> for RouterComponent {
    fn render(&self, area: Rect, buf: &mut Buffer, _props: ()) {
        match self.props.login_state {
            LoginState::NotLoggedIn | LoginState::LoggingIn | LoginState::LoginFailed => {
                self.login_page.render(area, buf, ());
            }
            LoginState::LoggedIn => {
                self.tab_page.render(area, buf, ());
            }
        }
    }
}
