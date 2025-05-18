/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use serde::Deserialize;
use thiserror::Error;

pub trait GitHubApi: Send + Sync {
    fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> impl Future<Output = Result<BranchComparison, ApiError>> + Send;
}

pub struct BranchComparisonRequest {
    pub repository_name: String,
    pub base_branch: String,
    pub head_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct BranchComparison {
    pub ahead_by: usize,
    pub behind_by: usize,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Repository not found")]
    RepositoryNotFound(),
}

pub struct DummyApi;

impl Default for DummyApi {
    fn default() -> Self {
        Self
    }
}

impl GitHubApi for DummyApi {
    async fn compare_commits(
        &self,
        _: BranchComparisonRequest,
    ) -> Result<BranchComparison, ApiError> {
        Ok(BranchComparison {
            ahead_by: 1,
            behind_by: 0,
        })
    }
}
