/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use crate::github_api::ApiError;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub struct ErrorHandlingRequest(reqwest::RequestBuilder);

impl ErrorHandlingRequest {
    pub async fn send(self) -> Result<ErrorHandlingResponse, ApiError> {
        self.0
            .send()
            .await
            .inspect_err(|error| tracing::error!(%error, "Sending request failed"))
            .map_err(|_| ApiError::Unspecific)
            .map(ErrorHandlingResponse)
    }
}

pub struct ErrorHandlingResponse(reqwest::Response);

impl ErrorHandlingResponse {
    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    pub fn is_success(&self) -> bool {
        self.status().is_success()
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T, ApiError> {
        if !self.is_json_content_type() {
            let content = self
                .0
                .text()
                .await
                .inspect_err(|error| tracing::error!(%error, "Retrieving text content failed"))
                .map_err(|_| ApiError::Unspecific)?;

            tracing::error!(content, "Content-Type is not valid for JSON");

            return Err(ApiError::Unspecific);
        }

        let body = self
            .0
            .bytes()
            .await
            .inspect_err(|error| tracing::error!(%error, "Retrieving byte content failed"))
            .map_err(|_| ApiError::Unspecific)?;

        serde_json::from_slice(&body)
            .inspect_err(|error| tracing::error!(%error, "Deserializing JSON content failed"))
            .map_err(|_| ApiError::Unspecific)
    }

    fn is_json_content_type(&self) -> bool {
        self.0
            .headers()
            .get("Content-Type")
            .map(|value| {
                value.as_bytes().starts_with(b"application/json")
                    || value.as_bytes().starts_with(b"application/vnd.github+json")
            })
            .unwrap_or(false)
    }
}

pub trait IntoErrorHandlingRequest {
    fn with_error_handling(self) -> ErrorHandlingRequest;
}

impl IntoErrorHandlingRequest for reqwest::RequestBuilder {
    fn with_error_handling(self) -> ErrorHandlingRequest {
        ErrorHandlingRequest(self)
    }
}
