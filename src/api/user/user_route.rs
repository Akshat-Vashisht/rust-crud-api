use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    api::user::user_handler::{
        get_me_handler, login_user_handler, logout_handler, register_user_handler,
    },
    jwtauth::auth,
    AppState,
};

pub fn user_routes(app_state: AppState) -> Router {
    Router::new()

        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/auth/getuser",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}
