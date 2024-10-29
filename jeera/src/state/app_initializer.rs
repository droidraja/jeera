use tokio::sync::mpsc::UnboundedSender;

use crate::api::JiraApi;

use super::{action::Action, config::Config, LoginState};

pub async fn try_login_config(action_tx: UnboundedSender<Action>, config: &Config) -> LoginState {
    action_tx
        .send(Action::LoginStatus(LoginState::LoggingIn))
        .unwrap();
    let jira_api = JiraApi::from(config);
    let current_user = jira_api.get_current_user().await;

    if let Ok(_) = current_user {
        action_tx
            .send(Action::LoginStatus(LoginState::LoggedIn))
            .unwrap();
        return LoginState::LoggedIn;
    } else {
        tracing::error!("Login failed ({:#?})", current_user);
        action_tx
            .send(Action::LoginStatus(LoginState::LoginFailed))
            .unwrap();
        return LoginState::LoginFailed;
    }
}

pub async fn try_login(action_tx: UnboundedSender<Action>) -> LoginState {
    let config = Config::load_config();
    if let Ok(config) = config {
        try_login_config(action_tx, &config).await
    } else {
        action_tx
            .send(Action::LoginStatus(LoginState::NotLoggedIn))
            .unwrap();
        LoginState::NotLoggedIn
    }
}

pub async fn try_login_with_credentials(
    action_tx: UnboundedSender<Action>,
    username: String,
    password: String,
    host: String,
) -> LoginState {
    let config = Config {
        email: username,
        api_token: password,
        host,
    };

    tracing::info!("Config: {:#?}", &config);
    let login_state = try_login_config(action_tx, &config).await;
    if login_state == LoginState::LoggedIn {
        config.write_config().unwrap();
    }
    login_state
}
