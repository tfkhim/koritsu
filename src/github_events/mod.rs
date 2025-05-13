/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::sync::Arc;

use axum::{body::Bytes, extract::State};
use hyper::HeaderMap;
use verifier::{EventSignature, EventVerifier};
use workflow_run::handle_workflow_run_event;

use crate::application_config::ApplicationConfig;

mod verifier;
mod workflow_run;

pub async fn event_handler(
    State(config): State<Arc<ApplicationConfig>>,
    headers: HeaderMap,
    body: Bytes,
) {
    let event_type = headers
        .get("X-GitHub-Event")
        .expect("Missing X-GitHub-Event header")
        .to_str()
        .unwrap();

    let sha256_signature = headers
        .get("X-Hub-Signature-256")
        .expect("Missing X-Hub-Signature-256 header")
        .to_str()
        .map(EventSignature::from_signature_header)
        .unwrap();

    let verifier = EventVerifier::new(&config.github_webhook_secret);

    if !verifier.payload_is_valid(&body, &sha256_signature) {
        panic!("Request signature is not valid");
    }

    if event_type == "workflow_run" {
        handle_workflow_run_event(serde_json::from_slice(&body).unwrap()).await;
    }
}
