use axum::Router;

use crate::app_state::AppState;

mod auth;
mod response;
mod swagger;
mod user;

pub fn get_routes(app_state: AppState) -> Router {
    let public_routes = auth::api::get_public_routes(app_state.clone());

    let private_routes = user::api::get_private_routes(app_state.clone())
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth::middleware::authorize,
        ));

    let admin_routes = user::api::get_admin_routes(app_state.clone())
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth::middleware::authorize_admin,
        ));

    swagger::get_routes()
        .merge(public_routes)
        .merge(private_routes)
        .merge(admin_routes)
}
