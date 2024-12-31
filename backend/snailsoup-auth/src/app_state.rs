use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    config::Config,
    services::{auth::AuthService, user::UserService},
};

#[derive(Clone)]
pub(super) struct AppState {
    pub config: Config,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(
        config: Config,
        auth_service: Arc<AuthService>,
        user_service: Arc<UserService>,
    ) -> AppState {
        AppState {
            config,
            auth_service,
            user_service,
        }
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(app_state: &AppState) -> Config {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(app_state: &AppState) -> Arc<AuthService> {
        app_state.auth_service.clone()
    }
}

impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(app_state: &AppState) -> Arc<UserService> {
        app_state.user_service.clone()
    }
}
