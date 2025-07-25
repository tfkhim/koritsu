/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::sync::Arc;

use crate::{
    application_context::ApplicationContext,
    github_api::{
        ApiError, AuthenticationMethod, BranchComparison, BranchComparisonRequest, GitHubApi,
        GitHubApiProvider, UpdateReferenceRequest,
    },
};
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
    head_sha: String,
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

pub struct WorkflowRunHandler<ApiProvider> {
    app_context: Arc<ApplicationContext<ApiProvider>>,
}

impl<ApiProvider: GitHubApiProvider> WorkflowRunHandler<ApiProvider> {
    pub fn new(app_context: Arc<ApplicationContext<ApiProvider>>) -> Self {
        Self { app_context }
    }

    pub async fn handle_event(&self, event: WorkflowRunEvent) -> Result<(), ApiError> {
        if self.is_successful(&event) {
            let repository_name = event.repository.full_name;
            let installation_id = event.installation.id;
            let default_branch = event.repository.default_branch;
            let head_sha = event.workflow_run.head_sha;

            if let Some(head_branch) = event.workflow_run.head_branch {
                tracing::info!(
                    repository_name,
                    installation_id,
                    default_branch,
                    head_branch,
                    head_sha,
                    "Processing successful workflow run event",
                );

                let auth_method = AuthenticationMethod::AppInstallation { installation_id };
                let github_api = self.app_context.github_api(auth_method).await?;

                let branch_comparison_request = BranchComparisonRequest {
                    repository_name: repository_name.clone(),
                    base_branch: default_branch.clone(),
                    head_branch,
                };

                let BranchComparison {
                    ahead_by,
                    behind_by,
                } = github_api
                    .compare_commits(branch_comparison_request)
                    .await?;

                tracing::info!(ahead_by, behind_by, "Branch comparison was successful");

                if ahead_by == 1 && behind_by == 0 {
                    tracing::info!("Performing fast forward merge");
                    let reference_update = UpdateReferenceRequest {
                        repository_name,
                        reference: format!("heads/{default_branch}"),
                        sha1: head_sha,
                        force: false,
                    };
                    github_api.update_reference(reference_update).await?;
                }
            }
        }

        Ok(())
    }

    fn is_successful(&self, event: &WorkflowRunEvent) -> bool {
        event.action == "completed"
            && event.workflow_run.conclusion.as_deref().unwrap_or("") == "success"
    }
}
