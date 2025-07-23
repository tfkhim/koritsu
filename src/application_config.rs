/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::env::{self, VarError};

#[derive(Clone)]
pub struct ApplicationConfig {
    pub github_base_url: String,
    pub github_webhook_secret: String,
    pub client_id: String,
    pub private_key_file: String,
}

impl ApplicationConfig {
    pub fn from_env() -> Result<ApplicationConfig, VarError> {
        Ok(ApplicationConfig {
            github_base_url: "https://api.github.com".to_owned(),
            github_webhook_secret: env::var("GITHUB_WEBHOOK_SECRET")?,
            client_id: env::var("GITHUB_CLIENT_ID")?,
            private_key_file: env::var("PRIVATE_KEY_FILE")?,
        })
    }
}
