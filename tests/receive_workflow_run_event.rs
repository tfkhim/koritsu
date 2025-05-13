/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::convert::Infallible;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    routing::RouterIntoService,
};
use hmac::{Hmac, Mac};
use koritsu_app::{ApplicationConfig, build_app};
use serde_json::{Value, json};
use sha2::Sha256;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn receive_workflow_run() {
    let mut client = TestClient::new();
    let payload = json!({
        "action": "completed",
        "workflow_run": {
            "conclusion": "success",
            "head_branch": "ready/new-feature",
        },
    });

    let response = client.send_workflow_run_event(&payload).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

struct TestClient {
    config: ApplicationConfig,
    service: RouterIntoService<Body>,
}

impl TestClient {
    fn new() -> Self {
        let config = ApplicationConfig {
            github_webhook_secret: "secret".to_string(),
        };

        let service = build_app(config.clone()).into_service();

        TestClient { config, service }
    }

    async fn send_workflow_run_event(
        &mut self,
        payload: &Value,
    ) -> Result<Response<Body>, Infallible> {
        let payload = serde_json::to_vec(payload).unwrap();
        let signature = self.compute_signature(&payload);

        let request = Request::builder()
            .method("POST")
            .uri("/github/events")
            .header("X-GitHub-Event", "workflow_run")
            .header("X-Hub-Signature-256", format!("sha256={}", signature))
            .body(Body::from(payload))
            .unwrap();

        self.service.ready().await.unwrap().call(request).await
    }

    fn compute_signature(&self, payload: &[u8]) -> String {
        let secret = self.config.github_webhook_secret.as_bytes();

        let signature = Hmac::<Sha256>::new_from_slice(secret)
            .unwrap()
            .chain_update(payload)
            .finalize()
            .into_bytes();

        signature
            .into_iter()
            .flat_map(|byte| [Self::byte_to_hex(byte >> 4), Self::byte_to_hex(byte)])
            .collect()
    }

    fn byte_to_hex(byte: u8) -> char {
        let encoding = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        encoding[(byte & 15u8) as usize]
    }
}
