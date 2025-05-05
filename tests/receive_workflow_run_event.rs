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
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn receive_workflow_run() {
    let app = koritsu_app::build_app();
    let payload = json!({
        "action": "completed",
        "workflow_run": {
            "conclusion": "success",
            "head_branch": "ready/new-feature",
        },
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/github/events")
                .header("X-GitHub-Event", "workflow_run")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
