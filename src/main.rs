/*
 * This file is part of koritsu
 *
 * Copyright (c) 2025 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::net::SocketAddr;
use thiserror::Error;

use koritsu_app::{ApplicationConfig, build_app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    let config = ApplicationConfig::from_env()?;
    let app = build_app(config);

    let address = "127.0.0.1:8080".parse::<SocketAddr>()?;

    let listener = TcpListener::bind(address)
        .await
        .map_err(|error| StartupError::UnableToBindToSocket(address, error))?;

    axum::serve(listener, app)
        .await
        .map_err(|error| StartupError::CouldNotServeApplication(address, error))?;

    Ok(())
}

#[derive(Error, Debug)]
enum StartupError {
    #[error("Could not load application configuration")]
    ConfigurationError(#[from] std::env::VarError),

    #[error("Invalid socket address")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("Unable to bind to socket {0}")]
    UnableToBindToSocket(SocketAddr, #[source] std::io::Error),

    #[error("Unable to serve application at socket {0}")]
    CouldNotServeApplication(SocketAddr, #[source] std::io::Error),
}
