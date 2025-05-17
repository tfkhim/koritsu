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
    repository: Repository,
    installation: Installation,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
    conclusion: Option<String>,
    head_branch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    full_name: String,
    default_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Installation {
    id: usize,
}

impl WorkflowRunEvent {
    fn is_successful(&self) -> bool {
        self.action == "completed"
            && self.workflow_run.conclusion.as_deref().unwrap_or("") == "success"
    }
}

pub async fn handle_workflow_run_event(event: WorkflowRunEvent) {
    if event.is_successful() {
        let repository_name = &event.repository.full_name;
        println!("Repository name is {repository_name}");

        let installation_id = &event.installation.id;
        println!("Installation id is {installation_id}");

        let default_branch = &event.repository.default_branch;
        println!("Respository default branch is {default_branch}");

        if let Some(branch) = event.workflow_run.head_branch {
            println!("Workflow run for branch {branch} successful");
        }
    }
}
