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
    github_api::{GitHubApi, GitHubApiProvider},
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

    pub fn github_api(&self) -> impl GitHubApi {
        self.github_api_provider.get_api()
    }
}
