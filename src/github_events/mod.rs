/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use axum::body::Bytes;
use hyper::HeaderMap;
use workflow_run::handle_workflow_run_event;

mod workflow_run;

pub async fn event_handler(headers: HeaderMap, body: Bytes) {
    let event_type = headers
        .get("X-GitHub-Event")
        .expect("Missing X-GitHub-Event header")
        .to_str()
        .unwrap();

    if event_type == "workflow_run" {
        handle_workflow_run_event(serde_json::from_slice(&body).unwrap()).await;
    }
}
