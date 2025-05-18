/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::{ApplicationConfig, github_api::GitHubApi};

pub struct ApplicationContext<API> {
    config: ApplicationConfig,
    github_api: API,
}

impl<API> ApplicationContext<API> {
    pub fn config(&self) -> &ApplicationConfig {
        &self.config
    }
}

impl<API: GitHubApi> ApplicationContext<API> {
    pub fn new(config: ApplicationConfig, github_api: API) -> Self {
        Self { config, github_api }
    }

    pub fn github_api(&self) -> &API {
        &self.github_api
    }
}
