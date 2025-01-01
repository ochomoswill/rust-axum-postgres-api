use std::sync::Arc;

use axum::{
    routing::{get, post, delete, patch},
    Router,
};
use crate::AppState;
use crate::notes::handler::{create_note_handler, delete_note_handler, edit_note_handler, get_note_handler, note_list_handler};

// Notes routes group
pub fn notes_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_note_handler))
        .route("/", get(note_list_handler))
        .route("/:id", get(get_note_handler))
        .route("/:id", patch(edit_note_handler))
        .route("/:id", delete(delete_note_handler))
        .with_state(app_state)
}
