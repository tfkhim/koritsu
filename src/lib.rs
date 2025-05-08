/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use axum::{Router, routing::post};
use github_events::event_handler;

mod github_events;

pub fn build_app() -> Router {
    Router::new().route("/github/events", post(event_handler))
}
