/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

pub use rest_impl::GitHubRestApiProvider;
use thiserror::Error;

mod rest_impl;

pub trait GitHubApiProvider: Send + Sync {
    fn get_api(
        &self,
        auth_method: AuthenticationMethod,
    ) -> impl Future<Output = Result<impl GitHubApi, ApiError>> + Send;
}

pub enum AuthenticationMethod {
    AppInstallation { installation_id: usize },
}

pub trait GitHubApi: Send + Sync {
    fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> impl Future<Output = Result<BranchComparison, ApiError>> + Send;

    fn update_reference(
        &self,
        request: UpdateReferenceRequest,
    ) -> impl Future<Output = Result<(), ApiError>> + Send;
}

pub struct BranchComparisonRequest {
    pub repository_name: String,
    pub base_branch: String,
    pub head_branch: String,
}

pub struct BranchComparison {
    pub ahead_by: usize,
    pub behind_by: usize,
}

pub struct UpdateReferenceRequest {
    pub repository_name: String,
    pub reference: String,
    pub sha1: String,
    pub force: bool,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Authentication(String),

    #[error("{0}")]
    Authorization(String),

    #[error("{0}")]
    RepositoryNotFound(String),

    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error("Unspecific error")]
    Unspecific,
}
