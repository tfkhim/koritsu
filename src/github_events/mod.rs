/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::sync::Arc;
use thiserror::Error;

use axum::{
    body::Bytes,
    extract::State,
    response::{IntoResponse, Response},
};
use hyper::{HeaderMap, StatusCode};
use serde_json::{Error as SerdeError, from_slice};
use verifier::{EventSignature, EventVerifier, SignatureConversionError};
use workflow_run::WorkflowRunHandler;

use crate::{
    application_context::ApplicationContext,
    github_api::{ApiError, GitHubApi},
    header_map_ext::{GetStrHeaderError, HeaderMapExt},
    problem::Problem,
};

mod verifier;
mod workflow_run;

pub async fn event_handler<API: GitHubApi>(
    State(app_context): State<Arc<ApplicationContext<API>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(), GithubEventError> {
    let signature_header = headers.get_str("X-Hub-Signature-256")?;
    let signature = EventSignature::from_signature_header(signature_header)?;

    let verifier = EventVerifier::new(&app_context.config().github_webhook_secret);

    if !verifier.payload_is_valid(&body, &signature) {
        return Err(GithubEventError::SignatureInvalid());
    }

    if headers.get_str("X-Github-Event")? == "workflow_run" {
        let handler = WorkflowRunHandler::new(app_context.github_api());
        handler.handle_event(from_slice(&body)?).await?;
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum GithubEventError {
    #[error(transparent)]
    InvalidHeader(#[from] GetStrHeaderError),

    #[error("Could not parse event signature")]
    InvalidSignatureHeader(#[from] SignatureConversionError),

    #[error("Event signature validation failed")]
    SignatureInvalid(),

    #[error("Event payload is invalid")]
    InvalidEventPayload(#[from] SerdeError),

    #[error(transparent)]
    ApiRequestFailed(#[from] ApiError),
}

impl IntoResponse for GithubEventError {
    fn into_response(self) -> Response {
        let status = match self {
            GithubEventError::SignatureInvalid() => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        };

        let detail = match self {
            GithubEventError::InvalidEventPayload(ref serde_error) => Some(serde_error),
            _ => None,
        };

        Problem::new(status, &self, detail).into_response()
    }
}
