use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use axum_extra::json;

use crate::{error::AppError, models::account::Account, services, AppState};

pub async fn read_logged(
    Extension(account): Extension<Account>,
) -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::OK,
        json!({
            "message": "Account details found",
            "logged_account": account
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        // FIXME (wedkarz): route path isn't the best but idk what to use here - figure
        //                  sth out later. Also, propably add some sort of DTO for the user.
        .route("/", get(read_logged))
        .layer(axum::middleware::from_fn(
            services::auth_services::auth_guard,
        ));

    // NOTE (wedkarz): raw CRUD propably shouldn't be exposed - use internally in
    //                 purpose-driven endpoints instead.
    // Router::new()
    //     .route("/", post(services::user::create_user))
    //     .route("/:id", get(services::user::read_user))
    //     .route("/", put(services::user::update_user))
    //     .route("/:id", delete(services::user::delete_user))
    //     .layer(Extension(state))
    Router::new()
        .merge(protected_routes)
        .layer(Extension(state))
}
