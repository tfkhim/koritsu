/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use hyper::{HeaderMap, header::ToStrError};
use thiserror::Error;

pub trait HeaderMapExt {
    fn get_str(&self, key: &'static str) -> Result<&str, GetStrHeaderError>;
}

impl HeaderMapExt for HeaderMap {
    fn get_str(&self, key: &'static str) -> Result<&str, GetStrHeaderError> {
        let value = self.get(key).ok_or(GetStrHeaderError::Missing(key))?;

        value
            .to_str()
            .map_err(|error| GetStrHeaderError::InvalidValue(key, error))
    }
}

#[derive(Error, Debug)]
pub enum GetStrHeaderError {
    #[error("Missing header {0}")]
    Missing(&'static str),

    #[error("Invalid value for header {0}")]
    InvalidValue(&'static str, #[source] ToStrError),
}
