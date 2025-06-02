/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::github_api::ApiError;
use crate::github_api::BranchComparison;
use crate::github_api::BranchComparisonRequest;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use std::ops::Deref;
use tracing::instrument;

use super::BasicError;

pub struct GithubCommitsRestApi<'a, C> {
    base_url: &'a str,
    client: C,
}

impl<'a, C: Deref<Target = Client>> GithubCommitsRestApi<'a, C> {
    pub fn new(base_url: &'a str, client: C) -> Self {
        Self { base_url, client }
    }
}

impl<C: Deref<Target = Client>> GithubCommitsRestApi<'_, C> {
    #[instrument(skip_all, fields(request))]
    pub async fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> Result<BranchComparison, ApiError> {
        let compare_url = format!(
            "{}/repos/{}/compare/{}...{}",
            self.base_url, request.repository_name, request.base_branch, request.head_branch
        );

        let response = self
            .client
            .get(&compare_url)
            .header("User-Agent", "koritsu-app")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .inspect_err(|error| tracing::error!(message = %error))
            .map_err(|_| ApiError::Unspecific)?;

        match response.status() {
            StatusCode::OK => response
                .json::<BranchComparisonRest>()
                .await
                .inspect_err(|error| tracing::error!(message = %error))
                .map_err(|_| ApiError::Unspecific)
                .map(|api_response| api_response.into()),
            StatusCode::NOT_FOUND => {
                let error_response = response
                    .json::<BasicError>()
                    .await
                    .inspect_err(|error| tracing::error!(message = %error))
                    .map_err(|_| ApiError::Unspecific)?;
                Err(ApiError::RepositoryNotFound(
                    error_response
                        .message
                        .unwrap_or_else(|| format!("Repository {} not found", compare_url)),
                ))
            }
            status => {
                let error_response = response
                    .json::<BasicError>()
                    .await
                    .inspect_err(
                        |error| tracing::error!(status = status.as_u16(), message = %error),
                    )
                    .map_err(|_| ApiError::Unspecific)?;

                tracing::error!(status = status.as_u16(), message = error_response.message);
                Err(ApiError::Unspecific)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BranchComparisonRest {
    pub ahead_by: usize,
    pub behind_by: usize,
}

impl From<BranchComparisonRest> for BranchComparison {
    fn from(api_response: BranchComparisonRest) -> Self {
        BranchComparison {
            ahead_by: api_response.ahead_by,
            behind_by: api_response.behind_by,
        }
    }
}
