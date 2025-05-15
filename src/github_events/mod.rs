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

use axum::{Json, body::Bytes, extract::State, response::IntoResponse};
use hyper::{HeaderMap, StatusCode};
use serde_json::{Error as SerdeError, from_slice};
use verifier::{EventSignature, EventVerifier, SignatureConversionError};
use workflow_run::handle_workflow_run_event;

use crate::{
    application_config::ApplicationConfig,
    header_map_ext::{GetStrHeaderError, HeaderMapExt},
    problem::Problem,
};

mod verifier;
mod workflow_run;

pub async fn event_handler(
    State(config): State<Arc<ApplicationConfig>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(), GithubEventError> {
    let signature_header = headers.get_str("X-Hub-Signature-256")?;
    let signature = EventSignature::from_signature_header(signature_header)?;

    let verifier = EventVerifier::new(&config.github_webhook_secret);

    if !verifier.payload_is_valid(&body, &signature) {
        return Err(GithubEventError::SignatureInvalid());
    }

    if headers.get_str("X-Github-Event")? == "workflow_run" {
        handle_workflow_run_event(from_slice(&body)?).await;
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
}

impl IntoResponse for GithubEventError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            GithubEventError::InvalidHeader(..) => StatusCode::BAD_REQUEST,
            GithubEventError::InvalidSignatureHeader(..) => StatusCode::BAD_REQUEST,
            GithubEventError::SignatureInvalid() => StatusCode::UNAUTHORIZED,
            GithubEventError::InvalidEventPayload(..) => StatusCode::BAD_REQUEST,
        };

        let problem = Json::<Problem>(self.into());
        (status, problem).into_response()
    }
}
