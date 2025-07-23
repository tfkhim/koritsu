/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::error::Error;
use std::fs;

use crate::ApplicationConfig;

use super::ApiError;
use super::AuthenticationMethod;
use super::BranchComparison;
use super::BranchComparisonRequest;
use super::GitHubApi;
use super::GitHubApiProvider;
use commits::GithubCommitsRestApi;
use error_handling::IntoErrorHandlingRequest;
use jwt_token_creator::JwtTokenCreator;
use reqwest::Client;
use serde::Deserialize;
use tracing::instrument;

mod commits;
mod error_handling;
mod jwt_token_creator;

pub struct GitHubRestApiProvider {
    token_creator: JwtTokenCreator,
    client: Client,
    base_url: String,
}

impl GitHubRestApiProvider {
    pub fn new(config: &ApplicationConfig) -> Result<Self, Box<dyn Error>> {
        let private_key_pem = fs::read_to_string(&config.private_key_file)?;
        let token_creator = JwtTokenCreator::new(config.client_id.clone(), &private_key_pem)?;

        let client = Client::new();
        let base_url = config.github_base_url.clone();

        Ok(Self {
            token_creator,
            client,
            base_url,
        })
    }
}

impl GitHubApiProvider for GitHubRestApiProvider {
    #[instrument(skip_all, fields(auth_method))]
    async fn get_api(&self, auth_method: AuthenticationMethod) -> Result<impl GitHubApi, ApiError> {
        let jwt_token = self
            .token_creator
            .build_token()
            .map_err(|e| ApiError::Authentication(e.to_string()))?;

        let AuthenticationMethod::AppInstallation { installation_id } = auth_method;
        let url = format!(
            "{}/app/installations/{installation_id}/access_tokens",
            self.base_url
        );
        let bearer_token = format!("Bearer {jwt_token}");

        let response = self
            .client
            .post(url)
            .header("User-Agent", "koritsu-app")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Authorization", bearer_token)
            .with_error_handling()
            .send()
            .await?;

        if response.is_success() {
            response
                .json::<AccessTokensRestResponse>()
                .await
                .map(|response| GitHubRestApi {
                    token: response.token,
                    client: &self.client,
                    base_url: &self.base_url,
                })
        } else {
            let basic_error: BasicError = response.json().await?;
            Err(ApiError::Authentication(
                basic_error
                    .message
                    .unwrap_or_else(|| "Could not retrieve access token".to_owned()),
            ))
        }
    }
}

pub type Token = String;

#[derive(Debug, Deserialize)]
struct AccessTokensRestResponse {
    token: String,
}

struct GitHubRestApi<'a> {
    token: Token,
    client: &'a Client,
    base_url: &'a str,
}

impl GitHubApi for GitHubRestApi<'_> {
    async fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> Result<BranchComparison, ApiError> {
        let comparison = GithubCommitsRestApi::new(&self.token, self.base_url, self.client)
            .compare_commits(request)
            .await?;

        Ok(comparison)
    }
}

#[derive(Debug, Deserialize)]
struct BasicError {
    pub message: Option<String>,
}
