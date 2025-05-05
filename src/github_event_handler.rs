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
use serde::Deserialize;

pub async fn github_event_handler(headers: HeaderMap, body: Bytes) {
    let event_type = headers
        .get("X-GitHub-Event")
        .expect("Missing X-GitHub-Event header")
        .to_str()
        .unwrap();

    if event_type == "workflow_run" {
        handle_workflow_run_event(serde_json::from_slice(&body).unwrap()).await;
    }
}

#[derive(Debug, Deserialize)]
struct WorkflowRunEvent {
    action: String,
    workflow_run: WorkflowRun,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    conclusion: Option<String>,
    head_branch: Option<String>,
}

impl WorkflowRunEvent {
    fn is_successful(&self) -> bool {
        self.action == "completed"
            && self.workflow_run.conclusion.as_deref().unwrap_or("") == "success"
    }
}

async fn handle_workflow_run_event(event: WorkflowRunEvent) {
    if event.is_successful() {
        if let Some(branch) = event.workflow_run.head_branch {
            println!("Workflow run for branch {branch} successful")
        }
    }
}
