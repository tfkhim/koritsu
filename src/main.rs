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
use tracing_subscriber::{
    EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    init_tracing();

    let config = ApplicationConfig::from_env()?;
    let app = build_app(config).map_err(StartupError::ApplicationInitialization)?;

    let address = "127.0.0.1:8080".parse::<SocketAddr>()?;

    let listener = TcpListener::bind(address)
        .await
        .map_err(|error| StartupError::UnableToBindToSocket(address, error))?;

    tracing::info!("listening on {}", address);

    axum::serve(listener, app)
        .await
        .map_err(|error| StartupError::CouldNotServeApplication(address, error))?;

    Ok(())
}

fn init_tracing() {
    let default_filter = |_| {
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    };

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(default_filter);

    let subscriber = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_file(true)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(subscriber)
        .init()
}

#[derive(Error, Debug)]
enum StartupError {
    #[error("Could not load application configuration")]
    Configuration(#[from] std::env::VarError),

    #[error(transparent)]
    ApplicationInitialization(Box<dyn std::error::Error>),

    #[error("Invalid socket address")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("Unable to bind to socket {0}")]
    UnableToBindToSocket(SocketAddr, #[source] std::io::Error),

    #[error("Unable to serve application at socket {0}")]
    CouldNotServeApplication(SocketAddr, #[source] std::io::Error),
}
