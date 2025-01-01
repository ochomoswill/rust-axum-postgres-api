use std::sync::Arc;
use axum::{
    Router,
};
use crate::{
    AppState,
};
use crate::notes::notes_router;

// API routes group
fn api_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/notes", notes_router(app_state))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", api_router(app_state))
}
