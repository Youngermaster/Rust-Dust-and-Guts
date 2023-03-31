use crate::handlers::*;
use axum::{
    handler::{delete, get, post, put},
    Router,
};

pub fn init_routes(app: &mut Router) {
    app.route("/", get(|| async { "Welcome to Axum MongoDB CRUD API!" }))
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user));
}
