/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::{
    ApplicationConfig,
    github_api::{ApiError, AuthenticationMethod, GitHubApi, GitHubApiProvider},
};

pub struct ApplicationContext<ApiProvider> {
    config: ApplicationConfig,
    github_api_provider: ApiProvider,
}

impl<ApiProvider> ApplicationContext<ApiProvider> {
    pub fn config(&self) -> &ApplicationConfig {
        &self.config
    }
}

impl<ApiProvider: GitHubApiProvider> ApplicationContext<ApiProvider> {
    pub fn new(config: ApplicationConfig, github_api_provider: ApiProvider) -> Self {
        Self {
            config,
            github_api_provider,
        }
    }

    pub async fn github_api(
        &self,
        auth_method: AuthenticationMethod,
    ) -> Result<impl GitHubApi, ApiError> {
        self.github_api_provider.get_api(auth_method).await
    }
}
