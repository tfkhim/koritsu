/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::ApplicationConfig;

use super::ApiError;
use super::BranchComparison;
use super::BranchComparisonRequest;
use super::GitHubApi;
use super::GitHubApiProvider;
use commits::GithubCommitsRestApi;
use reqwest::Client;
use serde::Deserialize;

mod commits;
mod error_handling;

pub struct GitHubRestApiProvider {
    client: Client,
    base_url: String,
}

impl GitHubRestApiProvider {
    pub fn new(config: &ApplicationConfig) -> Self {
        let client = Client::new();
        let base_url = config.github_base_url.clone();
        Self { client, base_url }
    }
}

impl GitHubApiProvider for GitHubRestApiProvider {
    fn get_api(&self) -> impl GitHubApi {
        GitHubRestApi {
            client: &self.client,
            base_url: &self.base_url,
        }
    }
}

struct GitHubRestApi<'a> {
    client: &'a Client,
    base_url: &'a str,
}

impl GitHubApi for GitHubRestApi<'_> {
    async fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> Result<BranchComparison, ApiError> {
        let comparison = GithubCommitsRestApi::new(self.base_url, self.client)
            .compare_commits(request)
            .await?;

        Ok(comparison)
    }
}

#[derive(Debug, Deserialize)]
struct BasicError {
    pub message: Option<String>,
}
