/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowRunEvent {
    action: String,
    workflow_run: WorkflowRun,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
    conclusion: Option<String>,
    head_branch: Option<String>,
}

impl WorkflowRunEvent {
    fn is_successful(&self) -> bool {
        self.action == "completed"
            && self.workflow_run.conclusion.as_deref().unwrap_or("") == "success"
    }
}

pub async fn handle_workflow_run_event(event: WorkflowRunEvent) {
    if event.is_successful() {
        if let Some(branch) = event.workflow_run.head_branch {
            println!("Workflow run for branch {branch} successful")
        }
    }
}
