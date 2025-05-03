/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use koritsu_app::build_app;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_github_events_endpoint() {
    let app = build_app();

    // Sample workflow_run payload
    let payload = json!({
        "action": "completed",
        "workflow_run": {
            "id": 123456789,
            "name": "Test Workflow",
            "conclusion": "success",
        },
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/github/events")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_github_events_endpoint_no_conclusion() {
    let app = build_app();

    let payload = json!({
        "action": "in_progress",
        "workflow_run": {
            "id": 987654321,
            "name": "Another Workflow",
            "conclusion": null,
        },
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/github/events")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
