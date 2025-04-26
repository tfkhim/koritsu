/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::io::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = koritsu_app::build_app();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
