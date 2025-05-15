/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::sync::Arc;

pub use application_config::ApplicationConfig;
use axum::{Router, routing::post};
use github_events::event_handler;

mod application_config;
mod github_events;
mod header_map_ext;
mod problem;

pub fn build_app(config: ApplicationConfig) -> Router {
    let config = Arc::new(config);

    Router::new()
        .route("/github/events", post(event_handler))
        .with_state(config)
}
