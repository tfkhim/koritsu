/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use axum::{Json, Router, routing::post};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkflowRunPayload {
    pub workflow_run: WorkflowRun,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRun {
    pub name: String,
    pub conclusion: Option<String>,
}

async fn github_events_handler(Json(payload): Json<WorkflowRunPayload>) {
    println!(
        "Workflow '{}' completed with status: {}",
        payload.workflow_run.name,
        payload
            .workflow_run
            .conclusion
            .as_deref()
            .unwrap_or("unknown")
    );
}

pub fn build_app() -> Router {
    Router::new().route("/github/events", post(github_events_handler))
}
