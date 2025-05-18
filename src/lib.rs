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
use application_context::ApplicationContext;
use axum::{Router, routing::post};
use github_api::{DummyApi, GitHubApi};
use github_events::event_handler;

pub mod github_api;

mod application_config;
mod application_context;
mod github_events;
mod header_map_ext;
mod problem;

pub fn build_app(config: ApplicationConfig) -> Router {
    let github_api = DummyApi;
    build_app_with_api(config, github_api)
}

pub fn build_app_with_api<API: GitHubApi + 'static>(
    config: ApplicationConfig,
    github_api: API,
) -> Router {
    let app_context = Arc::new(ApplicationContext::new(config, github_api));

    Router::new()
        .route("/github/events", post(event_handler))
        .with_state(app_context)
}
