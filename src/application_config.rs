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
    pub github_webhook_secret: String,
}

impl ApplicationConfig {
    pub fn from_env() -> Result<ApplicationConfig, VarError> {
        Ok(ApplicationConfig {
            github_webhook_secret: env::var("GITHUB_WEBHOOK_SECRET")?,
        })
    }
}
