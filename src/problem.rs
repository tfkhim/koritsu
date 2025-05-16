/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub struct Problem {
    title: String,
    status: StatusCode,
    detail: Option<String>,
}

impl Problem {
    pub fn new<T: ToString, D: ToString>(
        status: StatusCode,
        title: T,
        detail: Option<D>,
    ) -> Problem {
        Problem {
            title: title.to_string(),
            status,
            detail: detail.map(|inner| inner.to_string()),
        }
    }
}

impl IntoResponse for Problem {
    fn into_response(self) -> Response {
        let problem = ProblemResponse {
            title: self.title,
            status: self.status.into(),
            detail: self.detail,
        };

        (self.status, Json(problem)).into_response()
    }
}

#[derive(Deserialize, Serialize)]
struct ProblemResponse {
    title: String,

    status: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    detail: Option<String>,
}
