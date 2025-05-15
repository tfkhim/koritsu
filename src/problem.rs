/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Problem {
    title: String,
}

impl<E: Error> From<E> for Problem {
    fn from(error: E) -> Self {
        Problem {
            title: error.to_string(),
        }
    }
}
