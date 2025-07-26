/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::ops::Deref;

use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{HeaderValue, StatusCode},
    response::Response,
    routing::RouterIntoService,
};
use hmac::{Hmac, Mac};
use http_body_util::BodyExt;
use koritsu_app::{
    ApplicationConfig, build_app_with_api,
    github_api::{
        ApiError, AuthenticationMethod, BranchComparison, BranchComparisonRequest, GitHubApi,
        GitHubApiProvider, UpdateReferenceRequest,
    },
};
use serde_json::{Value, json};
use sha2::Sha256;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn returns_ok_for_a_valid_workflow_run() {
    let mut client = TestClient::new();
    let payload = given_successful_workflow_run_event_payload();

    let response = client.send_workflow_run_event(&payload).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.body().is_empty());
}

#[tokio::test]
async fn requires_the_signature_header() {
    let mut client = TestClient::new();
    let payload = given_successful_workflow_run_event_payload();
    let mut request = client.build_event_request("workflow_run", &payload);
    request.headers_mut().remove("X-Hub-Signature-256");

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.body_as_json(),
        json!({"status": 400, "title": "Missing header X-Hub-Signature-256"})
    );
}

#[tokio::test]
async fn requires_the_event_type_header() {
    let mut client = TestClient::new();
    let payload = given_successful_workflow_run_event_payload();
    let mut request = client.build_event_request("workflow_run", &payload);
    request.headers_mut().remove("X-GitHub-Event");

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.body_as_json(),
        json!({"status": 400, "title": "Missing header X-Github-Event"})
    );
}

#[tokio::test]
async fn returns_an_error_if_the_signature_is_invalid() {
    let mut client = TestClient::new();
    let payload = given_successful_workflow_run_event_payload();
    let mut request = client.build_event_request("workflow_run", &payload);
    request.headers_mut().insert(
        "X-Hub-Signature-256",
        HeaderValue::from_static("sha256=AFEB"),
    );

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.body_as_json(),
        json!({"status": 401, "title": "Event signature validation failed"})
    );
}

#[tokio::test]
async fn returns_an_error_if_the_event_payload_is_invalid() {
    let mut client = TestClient::new();
    let payload = json!({"invalid": "object"});
    let request = client.build_event_request("workflow_run", &payload);

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.body_as_json(),
        json!({
            "status": 400,
            "title": "Event payload is invalid",
            "detail": "missing field `action` at line 1 column 20"
        })
    );
}

#[tokio::test]
async fn returns_an_error_if_the_repository_is_not_known() {
    let mut client = TestClient::new();
    let payload = given_workflow_run_event_payload("ready/unknown");
    let request = client.build_event_request("workflow_run", &payload);

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.body_as_json(),
        json!({
            "status": 400,
            "title": "GitHub API request failed",
            "detail": "Repository not found",
        })
    );
}

#[tokio::test]
async fn returns_an_error_if_the_api_request_fails() {
    let mut client = TestClient::new();
    let payload = given_workflow_run_event_payload("ready/error");
    let request = client.build_event_request("workflow_run", &payload);

    let response = client.send_request(request).await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        response.body_as_json(),
        json!({
            "status": 500,
            "title": "GitHub API request failed",
            "detail": "Unspecific error",
        })
    );
}

fn given_successful_workflow_run_event_payload() -> Value {
    given_workflow_run_event_payload("read/new-feature")
}

fn given_workflow_run_event_payload(head_branch: &str) -> Value {
    json!({
        "action": "completed",
        "workflow_run": {
            "conclusion": "success",
            "head_branch": head_branch,
        },
        "repository": {
          "full_name": "test-owner/test-repo",
          "default_branch": "main",
        },
        "installation": {
          "id": 1337,
        },
    })
}

struct TestClient {
    config: ApplicationConfig,
    service: RouterIntoService<Body>,
}

impl TestClient {
    fn new() -> Self {
        let config = ApplicationConfig {
            github_base_url: String::default(),
            github_webhook_secret: "secret".to_owned(),
            client_id: String::default(),
            private_key_file: String::default(),
        };

        let api = TestGitHubApi {};
        let service = build_app_with_api(config.clone(), api).into_service();

        TestClient { config, service }
    }

    async fn send_workflow_run_event(&mut self, payload: &Value) -> Response<Bytes> {
        let request = self.build_event_request("workflow_run", payload);
        self.send_request(request).await
    }

    async fn send_request(&mut self, request: Request) -> Response<Bytes> {
        let (parts, body) = self
            .service
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap()
            .into_parts();

        let body_bytes = body.collect().await.unwrap().to_bytes();
        Response::from_parts(parts, body_bytes)
    }

    fn build_event_request(&self, event_type: &str, payload: &Value) -> Request {
        let payload = serde_json::to_vec(payload).unwrap();
        let signature = self.compute_signature(&payload);

        Request::builder()
            .method("POST")
            .uri("/github/events")
            .header("X-GitHub-Event", event_type)
            .header("X-Hub-Signature-256", format!("sha256={}", signature))
            .body(Body::from(payload))
            .unwrap()
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

trait ResponseExt {
    fn body_as_json(&self) -> Value;
}

impl ResponseExt for Response<Bytes> {
    fn body_as_json(&self) -> Value {
        serde_json::from_slice(self.body()).unwrap()
    }
}

struct TestGitHubApi;

impl GitHubApiProvider for TestGitHubApi {
    async fn get_api(&self, _: AuthenticationMethod) -> Result<impl GitHubApi, ApiError> {
        Ok(self)
    }
}

impl GitHubApi for &TestGitHubApi {
    async fn compare_commits(
        &self,
        request: BranchComparisonRequest,
    ) -> Result<BranchComparison, ApiError> {
        if request.head_branch.contains("unknown") {
            return Err(ApiError::RepositoryNotFound(
                "Repository not found".to_string(),
            ));
        }

        if request.head_branch.contains("error") {
            return Err(ApiError::Unspecific);
        }

        let (ahead_by, behind_by) = match request.head_branch.deref() {
            "ready/two_ahead" => (2, 0),
            "ready/one_ahead" => (1, 0),
            "ready/behind" => (1, 1),
            _ => (0, 0),
        };

        Ok(BranchComparison {
            ahead_by,
            behind_by,
        })
    }

    async fn update_reference(&self, _: UpdateReferenceRequest) -> Result<(), ApiError> {
        Ok(())
    }
}

impl From<&ApplicationConfig> for TestGitHubApi {
    fn from(_: &ApplicationConfig) -> Self {
        Self {}
    }
}
